pub async fn handler(input: Input) -> Result<(), Error> {
    let Input { mut i2c1, exti4, gpio_b, gpio_c } = input;
    // APDS-9960 interrupt events stream read from B4 pin.
    let mut exti4_stream = exti4.create_saturating_stream();
    let mut apds9960 = Apds9960Drv::init();
    let mut gestures = Gestures::init(&mut apds9960, &mut i2c1).await?;
    // Wait for a falling edge trigger on PB4.
    while exti4_stream.next().await.is_some() {
        // Repeat until PB4 is back to the high level.
        while !gpio_b.gpio_idr.load().idr4() {
            // Read APDS-9960 FIFO buffer.
            match gestures.advance(&mut apds9960, &mut i2c1).await? {
                // Turn on the LED.
                Some(Gesture::Up) => gpio_c.gpio_bsrr.store(|r| r.set_br13()),
                // Turn off the LED.
                Some(Gesture::Down) => gpio_c.gpio_bsrr.store(|r| r.set_bs13()),
                _ => {}
            }
        }
    }
    Ok(())
}
