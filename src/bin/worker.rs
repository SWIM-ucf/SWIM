use swim::emulation_core::agent::EmulationCoreAgent;
use yew_agent::Registrable;

fn main() {
    EmulationCoreAgent::registrar().register();
}
