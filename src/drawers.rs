use egui::{ComboBox, Label, Ui};
use egui_graphs::Graph;
use petgraph::{Undirected, graph::NodeIndex};

pub fn draw_section_graph(
    ui: &mut Ui,
    g: &mut Graph<(), f32, Undirected>,
    selected_node: Option<NodeIndex>,
    g_cb_dest_node_idx: &mut usize,
    g_te_edge_weight: &mut String,
) {
    ui.vertical_centered(|ui| {
        ui.label("Nodos");

        if ui.button("Agregar nodo").clicked() {
            g.add_node(());
        }

        ui.separator();

        ui.label("Conexiones");

        ui.add_visible(
            selected_node.is_none(),
            Label::new("Seleccione un nodo para agregar conexiones."),
        );

        ui.add_enabled_ui(selected_node.is_some(), |ui| {
            if selected_node.is_none() {
                return;
            }

            let selected_node_idx = selected_node.unwrap();

            let all_nodes: Vec<u32> = g
                .nodes_iter()
                .map(|n| n.0.index() as u32)
                .filter(|i| *i != selected_node_idx.index() as u32)
                .collect();

            let all_nodes_str: Vec<String> = all_nodes.iter().map(|n| n.to_string()).collect();

            ComboBox::from_label("Nodo destino").show_index(
                ui,
                g_cb_dest_node_idx,
                all_nodes_str.len(),
                |i| all_nodes_str.get(i).unwrap(),
            );

            ui.text_edit_singleline(g_te_edge_weight);

            let new_edge_weight = match g_te_edge_weight.parse::<f32>() {
                Ok(n) => n,
                Err(_) => 0.0,
            };

            if ui.button("Agregar conexión").clicked() {
                g.add_edge(
                    selected_node_idx,
                    NodeIndex::new(*all_nodes.get(*g_cb_dest_node_idx).unwrap() as usize),
                    new_edge_weight,
                );
            }
        })
    });
}
