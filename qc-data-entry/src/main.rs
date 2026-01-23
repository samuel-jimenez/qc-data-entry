use qc_data_entry::DataEntryConfig;

// mod config;

fn main() {
    // Print out our settings
    println!("{:?}", DataEntryConfig::load());
}
