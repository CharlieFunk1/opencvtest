use opencv::{
    highgui,
    prelude::*,
    Result,
    videoio,
};

fn main() -> Result<()> {
    //Creates window names video capture
    let window = "video capture";
    highgui::named_window(window, highgui::WINDOW_AUTOSIZE)?;
    //Create VideoCapture element "cap"
    let mut cap = videoio::VideoCapture::new(0, videoio::CAP_ANY)?;
    //If not open throw error
    let opened = videoio::VideoCapture::is_opened(&cap)?;
    if !opened {
	panic!("Unable to open video");
    }
    //video loop
    loop {
	//create Mat array "frame"
	let mut frame = Mat::default();
	//read next frame from cap
	cap.read(&mut frame)?;
	//Background subtraction attempt
	
	//let backSub = opencv::video::create_background_subtractor_mog2(1, 0.9, true).unwrap();
	//let mut fgMask = opencv::prelude::BackgroundSubtractorMOG2::apply(4, &frame, backSub, 0.9);

	//Display frame in window
	highgui::imshow(window, &mut frame)?;
	
	//Display fgMask in window    
	//highgui::imshow(window, &mut fgMask)?;
	
	//Press key to close
	let key = highgui::wait_key(30)?;
	if key > 0 && key != 255 {
	    break;
	}
    }
    Ok(())
}
