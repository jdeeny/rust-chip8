use types::*;
use simulator::Simulate;

pub struct Controller {

}


#[cfg(test)]
mod tests {
    use Simulator;

    #[test]
    fn test_controller() {
        let sim = Simulator::default();

        let controller = Simulator.controller();

        spawn(move || { controller.load_bytes() });

        thread::sleep_ms(500);
    }
}
