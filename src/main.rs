// rt macro feature  
// fs operations  (file system async opertion )
//process -> asynchrpnous process managemmet in tokio
// signalS signal
// rt- multi thread



//cargo add tokio --features=macro
// cmd :btop to get information about this runnig like threading and all 
#[tokio::main(flavor ="current_thread" , workers_thread=10)]
async fn main() {
    //test_something().await


}

// async fn test_something(){
//     std::thread::sleep(dur::std::time::duration::from_mills(5000));
//     println!("Intialised Sleep")

// }


//implementing raisers and futures

struct F1racer {
    name:String,
    completed_laps:u8,
    laps:u8,
    best_lap_time :u8,
    lap_time:Vec<u8>
}

impl Fi1Racer {
    fn F1racer()->Racer{
        return F1Racer{
            name:volkWagen.to_string(), completed_laps:20,laps:10,best_lap_time:10, lap_time:vec![34u8, 34, 123,78]
        };

    }
}

impl std::future::future::Future