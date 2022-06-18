use opencv::{
	highgui,
	prelude::*,
	Result,
	videoio,
};

fn main() -> Result<()> {
	let window = "video capture";
        highgui::named_window(window, highgui::WINDOW_AUTOSIZE)?;
	let mut cap = videoio::VideoCapture::from_file("video/newlong4.mov", videoio::CAP_ANY)?; 
	let opened = videoio::VideoCapture::is_opened(&cap)?;
	if !opened {
		panic!("Unable to open video");
	}
	loop {
		let mut frame = Mat::default();
		cap.read(&mut frame)?;
		if frame.size()?.width > 0 {
			highgui::imshow(window, &mut frame)?;
		}
		let key = highgui::wait_key(30)?;
		if key > 0 && key != 255 {
			break;
		}
	}
	Ok(())
}
