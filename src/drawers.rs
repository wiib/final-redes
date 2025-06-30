use egui::{Button, Label, Ui};
use egui_graphs::Graph;
use petgraph::{Undirected, graph::NodeIndex};

pub fn draw_section_graph(
    ui: &mut Ui,
    g: &mut Graph<(), f32, Undirected>,
    selected_node: Option<NodeIndex>,
) {
    ui.vertical_centered(|ui| {
        if ui.button("Agregar nodo").clicked() {
            g.add_node(());
        }

        ui.separator();

        ui.add_visible(selected_node.is_none(), Label::new("Seleccione un nodo"));

        let add_edge_button =
            ui.add_visible(selected_node.is_some(), Button::new("Agregar conexión"));

        if add_edge_button.clicked() {
            todo!("implement")
        }
    });
}
