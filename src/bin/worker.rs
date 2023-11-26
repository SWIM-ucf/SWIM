use yew_agent::Registrable;
use swim::emulation_core::agent::EmulationCoreAgent;

fn main() {
    EmulationCoreAgent::registrar().register();
}