use std::fs::File;
use std::path::{Path, PathBuf};

use anyhow::Result;

use bevy::{
  prelude::*,
};
use bevy_egui::{egui, EguiContexts};

use node_engine::{
  NodeGraph,
  NodeRegistry,
  NodeGraphCompile,
};

use crate::*;

#[derive(Resource, Clone, Debug)]
pub struct ShaderGraphEditor {
  pub title: String,
  pub size: egui::Vec2,
  pub graph: NodeGraph,
  open: bool,
  open_preview: bool,
  file: PathBuf,
  code: String,
  last_change_counter: usize,
  last_error_msg: Option<String>,
  changed: bool,
}

impl Default for ShaderGraphEditor {
  fn default() -> Self {
    let registry = NodeRegistry::build();
    let mut graph = NodeGraph::new();
    let root = registry
      .new_by_name("Fragment output");
    if let Some(root) = root.ok() {
      let output_id = graph.add(root);
      graph.set_output(Some(output_id));
    }

    Self {
      title: "Graph editor".to_string(),
      size: (1000., 300.).into(),
      graph,
      file: "shader_graph.json".into(),
      code: "".to_string(),
      last_change_counter: 0,
      last_error_msg: None,
      open: true,
      open_preview: true,
      changed: true,
    }
  }
}

impl ShaderGraphEditor {
  pub fn new() -> Self {
    Default::default()
  }

  pub fn load<P: AsRef<Path>>(&mut self, path: P) -> Result<()> {
    let path = path.as_ref();
    let file = File::open(path)?;
    self.file = path.into();
    self.graph = serde_json::from_reader(file)?;
    self.changed = true;
    self.last_change_counter = 0;
    Ok(())
  }

  pub fn save(&self) -> Result<()> {
    let file = File::create(&self.file)?;
    serde_json::to_writer_pretty(file, &self.graph)?;
    Ok(())
  }

  fn handle_result<V>(&mut self, context: &str, res: Result<V>) -> Result<V> {
    if let Err(err) = &res {
      let msg = format!("{context}: {err:?}");
      log::error!("{}", msg);
      self.last_error_msg = Some(msg);
    } else {
      self.last_error_msg = None;
    }
    res
  }

  pub fn toggle_open(&mut self) {
    self.open = !self.open;
  }

  pub fn toggle_open_preview(&mut self) {
    self.open_preview = !self.open_preview;
  }

  pub fn show(&mut self, ctx: &egui::Context) -> bool {
    let mut open = self.open;
    egui::Window::new(&self.title)
      .open(&mut open)
      .default_pos((0., 0.))
      .default_size(self.size)
      .show(ctx, |ui| {
        self.ui(ui);
      });
    self.open = open;
    self.changed
  }

  pub fn ui(&mut self, ui: &mut egui::Ui) {
    egui::TopBottomPanel::top("graph_top_panel").show_inside(ui, |ui| {
      egui::menu::bar(ui, |ui| {
        ui.menu_button("File", |ui| {
          if ui.button("Save").clicked() {
            let _ = self.handle_result("Failed to save", self.save());
            ui.close_menu();
          }
        });
      });
    });

    egui::TopBottomPanel::bottom("graph_bottom_panel").show_inside(ui, |ui| {
      if let Some(err_msg) = self.last_error_msg.as_ref() {
        ui.label(err_msg);
      }
    });
    // HACK: Without this side panel, the central panel will not work with a ScrollArea.
    egui::SidePanel::right("graph_right_panel")
      .min_width(0.)
      .frame(egui::Frame::none())
      .show_separator_line(false)
      .show_inside(ui, |_ui| {});

    let out = egui::CentralPanel::default().show_inside(ui, |ui| self.graph.ui(ui));
    if let Some(resp) = out.inner {
      // Graph menu.
      resp.context_menu(|ui| {
        if ui.button("Create Node").clicked() {
          self.graph.open_node_finder(ui);
          ui.close_menu();
        }
        if ui.button("Group Nodes").clicked() {
          self.graph.group_selected_nodes();
          ui.close_menu();
        }
      });
    }

    // Check if graph changed.
    let counter = self.graph.changed_counter();
    if counter != self.last_change_counter || self.code.is_empty() {
      self.last_change_counter = counter;
      self.generate_code();
    }
  }

  pub fn show_preview(&mut self, ctx: &egui::Context) -> bool {
    let mut open = self.open_preview;
    egui::Window::new(format!("{} Code", self.title))
      .open(&mut open)
      .default_pos((0., 400.))
      .default_size((700., 200.))
      .show(ctx, |ui| {
        self.preview_ui(ui);
      });
    self.open_preview = open;
    self.changed
  }

  fn preview_ui(&mut self, ui: &mut egui::Ui) {
    let Self { code, .. } = self;

    let theme = egui_extras::syntax_highlighting::CodeTheme::from_memory(ui.ctx());
    let mut layouter = |ui: &egui::Ui, string: &str, wrap_width: f32| {
      let mut layout_job =
          egui_extras::syntax_highlighting::highlight(ui.ctx(), &theme, string, "rs");
      layout_job.wrap.max_width = wrap_width;
      ui.fonts(|f| f.layout_job(layout_job))
    };

    egui::ScrollArea::vertical().show(ui, |ui| {
      let resp = ui.add(
        egui::TextEdit::multiline(code)
          .code_editor()
          .desired_rows(10)
          .desired_width(f32::INFINITY)
          .layouter(&mut layouter),
      );
      // Allow manual edits.
      if resp.changed() {
        self.changed = true;
      }
    });
  }

  fn generate_code(&mut self) {
    self.changed = true;
    let mut compiler = NodeGraphCompile::new();
    compiler.define_block("imports");
    compiler.define_block("bindings");

    if let Err(err) = compiler.compile_graph(&self.graph) {
      log::error!("Failed to compile shader graph: {err:?}");
    }

    self.code = compiler.dump();
  }

  pub fn recompile(&mut self) -> Option<Shader> {
    if !self.changed {
      return None;
    }
    self.changed = false;
    Some(Shader::from_wgsl(self.code.clone(), ""))
  }
}

pub fn shader_editor(
  input: Res<Input<KeyCode>>,
  mut editor: ResMut<ShaderGraphEditor>,
  mut contexts: EguiContexts,
  mut shaders: ResMut<Assets<Shader>>,
  mut materials: ResMut<Assets<StandardShaderGraphMaterial>>,
) {
  let ctx = contexts.ctx_mut();
  if !ctx.wants_keyboard_input() && input.just_pressed(KeyCode::S) {
    editor.toggle_open();
  }
  if !ctx.wants_keyboard_input() && input.just_pressed(KeyCode::C) {
    editor.toggle_open_preview();
  }
  editor.show(ctx);
  editor.show_preview(ctx);

  if let Some(shader) = editor.recompile() {
    let shader = shaders.add(shader);
    for (_, mat) in materials.iter_mut() {
      mat.extension.graph.fragment = Some(shader.clone());
    }
  }
}
