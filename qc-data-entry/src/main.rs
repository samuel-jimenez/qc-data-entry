use clap::Parser;
use log::info;
use log_result::ResultLog;
// use log::{error, info};
use qc_data_entry::{init_logger, DataEntryConfig, ProductLine, TopLevelArgs, DB};

fn main() {
    //parse args
    // let args: Vec<String> = env::args().collect();
    // cf confy clap

    let args = TopLevelArgs::parse();

    println!("{:?}", args);

    if args.verbose {
        println!("verbose");
    }
    if args.force {
        println!("force");
    }
    println!("name{:?}", args.name);
    // println!("comand{:?}", options.comand);
    println!("reqcomand {:?}", args.reqcomand);

    // std::process::exit(0);

    //load config
    let config = DataEntryConfig::load_from(args.config);
    println!("Using config: {:?}", config);

    // // log to file
    init_logger(config.log_file.clone()).unwrap();
    info!("Using config: {}", config.config_path().display());

    //open_db
    let qc_db = DB::new(config.db_file()).error().unwrap();

    println!(
        "product_info: {:?}",
        ProductLine::select_product_info_all(&qc_db) // qc_db.select_product_info_all().unwrap()
    );

    //
    // //setup print goroutine
    // threads.PRINT_QUEUE = make(chan string, 4)
    // defer close(threads.PRINT_QUEUE)
    // go threads.Do_print_queue(threads.PRINT_QUEUE)
    //
    // //setup status_bar goroutine
    // threads.STATUS_QUEUE = make(chan string, 16)
    // defer close(threads.STATUS_QUEUE)
    // go threads.Do_status_queue(threads.STATUS_QUEUE)

    config.save().unwrap();
}
