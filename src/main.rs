#![recursion_limit = "1024"]

#[macro_use]
extern crate error_chain;

extern crate clap;
extern crate num;
extern crate image;
extern crate rayon;
extern crate num_cpus;


mod errors {
    error_chain! {
        foreign_links {
            Io(::std::io::Error);
            Num(::std::num::ParseFloatError);
        }
    }
}

use errors::*;
use errors::Result;


mod fractal;

use std::fs::File;
use clap::{App, Arg};
use image::png::PNGEncoder;
use image::ColorType;
use num::complex::Complex64;
use fractal::*;


fn main() {
    let result = run();

    if let Err(ref e) = result {
        println!("error: {}", e);

        for e in e.iter().skip(1) {
            println!("caused by: {}", e);
        }

        if let Some(backtrace) = e.backtrace() {
            println!("backtrace: {:?}", backtrace);
        }

        ::std::process::exit(1);
    }
}


fn run() -> Result<()> {
    let arguments = App::new("fractal")
        .version("0.1")
        .author("sargon@me.com")
        .about("Creates awesome fractal images")
        .arg(Arg::with_name("type")
            .short("t")
            .long("type")
            .conflicts_with("center")
            .help("Sets a region type (values: Default, SeaHorseValley)")
            .takes_value(true)
            .value_name("TYPE")
            .default_value("Default"))
        .arg(Arg::with_name("center-and-radius")
            .short("c")
            .long("center-and-radius")
            .conflicts_with("type")
            .help("Specifies the center and radius of the region (format: <real>/<imag>@radius (i.e. -0.74548/0.11669@0.01276))")
            .takes_value(true)
            .value_name("CENTER-AND-RADIUS"))
        .arg(Arg::with_name("max-iterations")
            .short("i")
            .long("max-iterations")
            .help("Sets max iterations")
            .takes_value(true)
            .value_name("MAX-ITERATIONS")
            .default_value("255"))
        .arg(Arg::with_name("resolution")
            .short("r")
            .long("resolution")
            .help("Specifies the resolution of the image (format: <width>x<height>)")
            .value_name("RESOLUTION")
            .default_value("1024x768"))
        .arg(Arg::with_name("parallel")
            .short("p")
            .long("parallel")
            .help("If specified the calculation is done in parallel"))
        .arg(Arg::with_name("band-height")
            .short("b")
            .long("band-height")
            .help("Specifies number of rows per band (only sensible in case of parallel execution of recursive engine)")
            .value_name("BAND-HEIGHT")
            .default_value("64"))
        .arg(Arg::with_name("engine")
            .short("e")
            .long("engine")
            .help("Choose engine to to use (values: Default, Recursive)")
            .takes_value(true)
            .value_name("ENGINE")
            .default_value("Default")
        )
        .arg(Arg::with_name("output-filename")
            .short("o")
            .long("--output-filename")
            .help("Specifies the output file name of the image")
            .required(true)
            .takes_value(true)
            .value_name("OUTPUT-FILENAME"))
        .get_matches();

    let (width, height) = parsed_resolution(arguments.value_of("resolution").unwrap()).chain_err(|| "parsing resolution failed")?;
    let max_iterations = parsed_max_iterations(arguments.value_of("max-iterations").unwrap()).chain_err(|| "parsing max iterations failed")?;
    let region = parsed_region(arguments.value_of("type"), arguments.value_of("center-and-radius"), width, height, max_iterations).chain_err(|| "parsing region failed")?;


    let engine = parsed_engine(arguments.value_of("engine").unwrap(),
                               arguments.is_present("parallel"),
                               arguments.value_of("band-height").unwrap())
        .chain_err(|| "parsing engine type failed")?;

    let output_filename = arguments.value_of("output-filename").unwrap();

    let mandelbrot = Mandelbrot::new(region);
    create_mandelbrot_file(&mandelbrot, &*engine, output_filename)?;

    Ok(())
}


fn create_mandelbrot_file(mandelbrot: &Mandelbrot, engine: &MandelbrotEngine, output_filename: &str) -> Result<()> {
    let pixels = mandelbrot.calculate(engine);

    let output = File::create(output_filename)?;
    let png_encoder = PNGEncoder::new(output);
    png_encoder.encode(pixels.as_slice(), mandelbrot.region.width_in_pixels as u32, mandelbrot.region.height_in_pixels as u32, ColorType::Gray(8))?;

    Ok(())
}


fn parsed_resolution(resolution: &str) -> Result<(u32, u32)> {
    let tokens: Vec<&str> = resolution
        .split('x')
        .collect();

    if tokens.len() != 2 {
        bail!("format for resolution should be: <width>x<height>");
    }

    let width = tokens[0].parse::<u32>().chain_err(|| "invalid width")?;
    let height = tokens[1].parse::<u32>().chain_err(|| "invalid height")?;

    Ok((width, height))
}


fn parsed_max_iterations(max_iterations: &str) -> Result<u8> {
    max_iterations
        .parse::<u8>()
        .chain_err(|| "invalid max iterations")
}


fn parsed_region(region_type: Option<&str>, center_and_radius: Option<&str>, width: u32, height: u32, max_iterations: u8) -> Result<Region> {
    if let Some(region_type) = region_type {
        let region_type = match region_type {
            "SeaHorseValley" => RegionType::SeaHorseValley,
            "Default" => RegionType::Default,
            _ => bail!("unsupported region type")
        };

        return Ok(Region::new_for_type(region_type, width, height, max_iterations));
    }

    if let Some(center_and_radius) = center_and_radius {
        let separators: &[char] = &['/', '@'];
        let tokens: Vec<&str> = center_and_radius.split(separators).collect();

        if tokens.len() != 3 {
            bail!("format for center and radius should be: <real>/<imag>@radius" );
        }

        let center_re = tokens[0].parse::<f64>().chain_err(|| "invalid center_re")?;
        let center_im = tokens[1].parse::<f64>().chain_err(|| "invalid center_im")?;
        let radius = tokens[2].parse::<f64>().chain_err(|| "invalid radius")?;

        return Ok(Region::new_for_center(Complex64::new(center_re, center_im), radius, width, height, max_iterations));
    }

    bail!("either region or center/radius have to be specified");
}


fn parsed_engine(engine_type: &str, in_parallel: bool, band_height: &str) -> Result<Box<MandelbrotEngine>> {
    match engine_type {
        "Default" => Ok(Box::new(SimpleMandelbrotEngine::new(in_parallel))),
        "Recursive" => {
            let band_height = band_height.parse::<u32>().chain_err(|| "invalid band size")?;

            Ok(Box::new(RecursiveMandelbrotEngine::new(in_parallel, band_height)))
        },
        _ => bail!("unsupported engine type")
    }
}
