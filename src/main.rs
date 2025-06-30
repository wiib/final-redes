use eframe::{App, CreationContext, run_native};
use egui::{CollapsingHeader, Context, ScrollArea};
use egui_graphs::{
    Graph, GraphView, LayoutRandom, LayoutStateRandom, SettingsInteraction, SettingsNavigation,
    SettingsStyle,
};
use petgraph::{
    Undirected,
    graph::EdgeIndex,
    stable_graph::{StableGraph, StableUnGraph},
};

mod drawers;

pub struct UndirectedApp {
    g: Graph<(), f32, Undirected>,
}

impl UndirectedApp {
    fn new(_: &CreationContext<'_>) -> Self {
        let g = generate_graph();
        Self { g: Graph::from(&g) }
    }
}

impl App for UndirectedApp {
    fn update(&mut self, ctx: &Context, _: &mut eframe::Frame) {
        egui::SidePanel::right("right_panel")
            .min_width(250.)
            .max_width(500.)
            .show(ctx, |ui| {
                ScrollArea::vertical().show(ui, |ui| {
                    CollapsingHeader::new("Grafo")
                        .default_open(true)
                        .show(ui, |ui| drawers::draw_section_graph(ui, &mut self.g));
                })
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            let interaction_settings = &SettingsInteraction::new().with_dragging_enabled(true);
            let style_settings = &SettingsStyle::new().with_labels_always(true);
            let navigation_settings = &SettingsNavigation::new()
                .with_zoom_and_pan_enabled(true)
                .with_fit_to_screen_enabled(true);

            ui.add(
                &mut GraphView::<_, _, _, _, _, _, LayoutStateRandom, LayoutRandom>::new(
                    &mut self.g,
                )
                .with_interactions(interaction_settings)
                .with_navigations(navigation_settings)
                .with_styles(style_settings),
            );
        });

        // make edge labels show edge weight
        for idx in 0..self.g.edge_count() {
            let edge = self.g.edge_mut(EdgeIndex::new(idx)).unwrap();
            edge.set_label(format!("{}", edge.props().payload));
        }
    }
}

fn generate_graph() -> StableGraph<(), f32, Undirected> {
    let mut g = StableUnGraph::default();

    let a = g.add_node(());
    let b = g.add_node(());
    let c = g.add_node(());

    g.add_edge(a, b, 5.);
    g.add_edge(b, c, 10.);
    g.add_edge(c, a, 15.);

    g
}

fn main() {
    let native_options = eframe::NativeOptions::default();
    run_native(
        "egui_graphs_undirected_demo",
        native_options,
        Box::new(|cc| Ok(Box::new(UndirectedApp::new(cc)))),
    )
    .unwrap();
}
