use photon_rs;


#[test]
fn grayscale_and_crop_non_alpha() {
    let mut photon_image = photon_rs::native::open_image("./tests/samples/fullImage.jpg").unwrap();
    let output_photon = photon_rs::transform::grayscale_and_crop(&mut photon_image);
    photon_rs::native::save_image(output_photon.clone(), "./tests/samples/fullImageOutput.jpg");
    assert_eq!(photon_image.get_height(), output_photon.get_height());
}



#[test]
fn grayscale_and_crop_alpha() {
    let mut photon_image = photon_rs::native::open_image("./tests/samples/pngwing.com.png").unwrap();
    let output_photon = photon_rs::transform::grayscale_and_crop(&mut photon_image);
    photon_rs::native::save_image(output_photon.clone(), "./tests/samples/alphaOut.png");
    assert_eq!(output_photon.get_height(), 772);
}