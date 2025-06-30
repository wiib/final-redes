use egui::Ui;
use egui_graphs::Graph;
use petgraph::{Undirected, visit::NodeRef};

pub fn draw_section_graph(ui: &mut Ui, g: &mut Graph<(), f32, Undirected>) {
    for node in g.nodes_iter() {
        ui.label(format!("Nodo: {}", node.id().index()));
    }
}
