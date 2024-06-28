use chrono::{DateTime, Local};
use egui::{pos2, vec2, Align2, Color32, FontId, Frame, Id, Pos2, Rect, Response, Sense, Stroke, TextureHandle, Vec2};
use std::time::{Duration, Instant, SystemTime};
use image::GenericImageView;
use std::collections::HashMap;

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct TemplateApp {
    label: String,
    pos: Vec<Vec<Pos2>>,
    #[serde(skip)]
    value: f32,
    time: Vec<String>,
    #[serde(skip)]
    response: Vec<Vec<Response>>,
    rect: Vec<Rect>,
    count: Vec<usize>,
    #[serde(skip)]
    image_texture: Vec<Option<TextureHandle>>,
    #[serde(skip)]
    timer_start: HashMap::<usize,Option<Instant>>,
    #[serde(skip)]
    buttons: Vec<bool>,
    hover: usize,
    #[serde(skip)]
    dragged_image: Option<usize>,
    selection: Option<Item>,
    bao_pos: Vec<Pos2>,
    shrimp_pos: Vec<Pos2>,
    #[serde(skip)]
    steamer: Vec<Vec<Option<Item>>>,
    show_inside:bool
}

#[derive(serde::Deserialize, serde::Serialize, Copy, Clone, Debug)]
pub enum Item {
    Xiaolongbao,
    Shrimp,
    ChickDumpling,
    FriedBun
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            label: "Hello World!".to_owned(),
            value: 2.7,
            pos: vec![vec![Pos2 { x: 700.0, y: 750.0 }], vec![Pos2 { x: 500.0, y: 750.0 }], vec![Pos2 { x: 300.0, y: 750.0 }], vec![Pos2 { x: 100.0, y: 750.0 }]],
            time: Vec::new(),
            response: vec![Vec::new(); 4],
            rect: Vec::new(),
            count: vec![0; 4],
            image_texture: Vec::new(),
            timer_start: Default::default(),
            buttons: vec![false; 20],
            hover: 0,
            bao_pos: vec![Pos2 { x: 500.0, y: 500.0 }],
            shrimp_pos: vec![Pos2 { x: 700.0, y: 750.0 }],
            dragged_image: None,
            selection: None,
            steamer: vec![Vec::new(); 20],
            show_inside:false,
        }
    }
}

impl TemplateApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let mut app: TemplateApp = Default::default();
        let image_path = vec!["./assets/shrimpdumpling.png", "./assets/xiaolongbao.png", "./assets/chicken.png", "./assets/bun.png"];
        for img in 0..image_path.len() {
            if let Ok(image) = image::open(image_path[img]) {
                let size = [image.width() as usize, image.height() as usize];
                let image_buffer = image.to_rgba8();
                let pixels: Vec<Color32> = image_buffer
                    .pixels()
                    .map(|p| Color32::from_rgba_premultiplied(p.0[0], p.0[1], p.0[2], p.0[3]))
                    .collect();
                let color_image = egui::ColorImage {
                    size,
                    pixels,
                };

                let texture_handle = cc.egui_ctx.load_texture(
                    "xiaolongbao",
                    color_image,
                    egui::TextureOptions::default(),
                );

                app.image_texture.push(Some(texture_handle));
            }
        }
        app
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Location {
    col: usize,
    row: usize,
}

impl eframe::App for TemplateApp {
    fn save(&mut self, storage: &mut dyn eframe::Storage) {}
    
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical(|ui| {

                for row in 0..5 {
                    ui.horizontal(|ui| {
                        for col in 1..6 {
                            let but_index = row * 5 + col;
                            if but_index < 19 {
                                let s = Stroke {
                                    width: 2.0,
                                    color: Color32::from_rgb(127, 127, 127)
                                };
                                let button = if self.buttons[but_index] {
                                    egui::Button::new(but_index.to_string())
                                        .stroke(s)
                                } else {
                                    egui::Button::new(but_index.to_string())
                                };
                                let button = ui.add_sized(
                                    [75.0, 75.0],
                                    button,
                                );
                               
                            button.context_menu(|ui| {
                                    ui.menu_button("Add", |ui| {
                                        if ui.button("Shrimp").clicked() {
                                           self.selection = Some(Item::Shrimp);
                                           self.timer_start.insert(but_index,Some(Instant::now()));
                                           ui.close_menu();
                                        }
                                        if ui.button("XiaoLongBao").clicked() {
                                            self.selection = Some(Item::Xiaolongbao);
                                            self.timer_start.insert(but_index,Some(Instant::now()));
                                            ui.close_menu();
                                        }
                                   
                                        if ui.button("Chicken").clicked() {
                                            self.selection = Some(Item::ChickDumpling);    
                                           self.timer_start.insert(but_index,Some(Instant::now()));
                                            ui.close_menu();
                                        }
                                     
                                        if ui.button("FriedBun").clicked() {
                                            self.selection = Some(Item::FriedBun);
                                            self.timer_start.insert(but_index,Some(Instant::now()));
                                            ui.close_menu();
                                        }
                                        if let Some(selection) = self.selection {
                                                self.steamer[but_index].push(Some(selection));
                                                self.selection = None;
                                                self.timer_start.insert(but_index,Some(Instant::now()));

                                         }
                                         
                                    });
                                    
                                });
                              
                                if but_index == 3 || but_index == 8 || but_index == 13 || but_index == 18 {
                                    ui.separator();
                                }
                                
                                if button.clicked() {
                                    self.buttons = vec![false; 20];
                                    self.buttons[but_index] = true;
                                    if let Some(selection) = self.selection {
                                        self.steamer[but_index].push(Some(selection));
                                        self.selection = None;
                                        self.timer_start.insert(but_index,Some(Instant::now()));
                                    
                                    }

                                }
                                
                                // Render images on buttons
                                for item in self.steamer[but_index].iter() {
                                    match item {
                                        Some(select) => {
                                            match select {
                                                Item::Shrimp => {
                                                    if let Some(texture) = &self.image_texture[0] {
                                                        ui.painter().image(
                                                            texture.id(),
                                                            button.rect,
                                                            Rect::from_min_max(pos2(0.0, 0.0), pos2(1.0, 1.0)),
                                                            Color32::WHITE,
                                                        );
                                                    }
                                                    if let Some(start_time) = self.timer_start.get(&but_index) {
                                                        let elapsed = start_time.unwrap().elapsed();
                                                   
                                                        
                                                   
                                                    let min: i32 = (elapsed.as_secs_f32() / 60.0) as i32;
                                                
                                                    let time=format!("{}m {:.0} s", min, elapsed.as_secs_f32() % 60.0);
                                                    let pos=Pos2{x:  button.rect.min.x+25.0,y:  button.rect.max.y+20.0};
                                                    
                                                    ui.painter().text(pos,Align2::CENTER_BOTTOM,time,FontId::default(),Color32::BLACK);
                                                }
                                                }
                                                Item::Xiaolongbao => {
                                                    if let Some(texture) = &self.image_texture[1] {
                                                        ui.painter().image(
                                                            texture.id(),
                                                            button.rect,
                                                            Rect::from_min_max(pos2(0.0, 0.0), pos2(1.0, 1.0)),
                                                            Color32::WHITE,
                                                        );
                                                    }
                                                    if let Some(start_time) = self.timer_start.get(&but_index) {
                                                        let elapsed = start_time.unwrap().elapsed();
                                                    let min: i32 = (elapsed.as_secs_f32() / 60.0) as i32;
                                                    let time=format!("{}m {:.0} s", min, elapsed.as_secs_f32() % 60.0);
                                               
                                                    let pos=Pos2{x:  button.rect.min.x+25.0,y:  button.rect.max.y+20.0};
                                                    ui.painter().text(pos,Align2::CENTER_BOTTOM,time,FontId::default(),Color32::BLACK);
                                                }
                                                }
                                                Item::ChickDumpling => {
                                                    if let Some(texture) = &self.image_texture[2] {
                                                        ui.painter().image(
                                                            texture.id(),
                                                            button.rect,
                                                            Rect::from_min_max(pos2(0.0, 0.0), pos2(1.0, 1.0)),
                                                            Color32::WHITE,
                                                        );
                                                        
                                                    }
                                                    if let Some(start_time) = self.timer_start.get(&but_index) {
                                                        let elapsed = start_time.unwrap().elapsed();
                                                   
                                                        
                                                   
                                                    let min: i32 = (elapsed.as_secs_f32() / 60.0) as i32;
                                                
                                                    let time=format!("{}m {:.0} s", min, elapsed.as_secs_f32() % 60.0);
                                                    let pos=Pos2{x:  button.rect.min.x+25.0,y:  button.rect.max.y+20.0};
                                                    
                                                    ui.painter().text(pos,Align2::CENTER_BOTTOM,time,FontId::default(),Color32::BLACK);
                                                }
                                                }
                                                Item::FriedBun => {
                                                    if let Some(texture) = &self.image_texture[3] {
                                                        ui.painter().image(
                                                            texture.id(),
                                                            button.rect,
                                                            Rect::from_min_max(pos2(0.0, 0.0), pos2(1.0, 1.0)),
                                                            Color32::WHITE,
                                                        );
                                                    }
                                                    if let Some(start_time) = self.timer_start.get(&but_index) {
                                                        let elapsed = start_time.unwrap().elapsed();
                                                   
                                                        
                                                   
                                                    let min: i32 = (elapsed.as_secs_f32() / 60.0) as i32;
                                                
                                                    let time=format!("{}m {:.0} s", min, elapsed.as_secs_f32() % 60.0);
                                                    let pos=Pos2{x:  button.rect.min.x+25.0,y:  button.rect.max.y+20.0};
                                                    
                                                    ui.painter().text(pos,Align2::CENTER_BOTTOM,time,FontId::default(),Color32::BLACK);
                                                }
                                                }
                                            }
                                        }
                                        None => {}
                                    }
                                }
                     
                          
                                for i in 0..4 {
                                    let count=self.count[i];
                                    if count >0{
                                    let index=count-1;
                                    
                                    let mut pos=self.pos[i][index];
                                    pos=Pos2{x:pos.x+37.5,y:pos.y+37.5};
                      
                                    if button.rect.contains(pos) {
                                        if let Some(selection) = self.selection {
                                            self.steamer[but_index].push(Some(selection) );
                                            self.pos[i][index]= self.pos[i][self.count[i]];
                                            self.timer_start.insert(but_index,Some(Instant::now()));

                                            if let Some(start_time) = self.timer_start.get(&but_index) {
                                                let elapsed = start_time.unwrap().elapsed();
                                   
                                            let min: i32 = (elapsed.as_secs_f32() / 60.0) as i32;
                                        
                                            let time=format!("{}m {:.0} s", min, elapsed.as_secs_f32() % 60.0);
                                            let pos=Pos2{x:  button.rect.min.x+25.0,y:  button.rect.max.y+20.0};
                                            
                                            ui.painter().text(pos,Align2::CENTER_BOTTOM,time,FontId::default(),Color32::BLACK);
                                        }
                                        }
                                      
                                    
                                    }
                                }
                                }
                            }
                        }
                    });
                        ui.add_space(20.0);   
                         
                }
            });

     
            
            let widget_size = Vec2::new(75.0, 75.0);
                                
            
     
            for i in 0..4 {
                

                let rects = Rect::from_min_size(self.pos[i][self.count[i]], widget_size);

                let responses = ui.allocate_rect(rects, egui::Sense::click_and_drag());

                if let Some(texture) = &self.image_texture[i] {
                    ui.painter().image(texture.id(), rects, Rect::from_min_max(pos2(0.0, 0.0), pos2(1.0, 1.0)), Color32::WHITE);
                }
             
                if responses.drag_started() {
                    self.selection = None;
                    let pos = self.pos[i][self.count[i]].clone();
                    self.pos[i].push(pos);
                    self.rect.push(Rect::from_min_size(self.pos[i][self.count[i]].clone(), widget_size));
                    self.response[i].push(responses.clone());
         
                    self.count[i] += 1;
                }
                
                if responses.drag_stopped() {
                    match i {
                        0 => self.selection = Some(Item::Shrimp),
                        1 => self.selection = Some(Item::Xiaolongbao),
                        2 => self.selection = Some(Item::ChickDumpling),
                        3 => self.selection = Some(Item::FriedBun),
                        _ => {}
                    };

                }
                
                if responses.dragged() {
                    self.pos[i][self.count[i]-1] += responses.drag_delta();
                }
                
                if responses.clicked() {
                    match i {
                        0 => self.selection = Some(Item::Shrimp),
                        1 => self.selection = Some(Item::Xiaolongbao),
                        2 => self.selection = Some(Item::ChickDumpling),
                        3 => self.selection = Some(Item::FriedBun),
                        _ => {}
                    };
                }

          
              
                for r in 0..self.count[i] {
                    let rect = Rect::from_min_size(self.pos[i][r], widget_size);
                    if !self.response[i].is_empty() {
                        self.response[i][r] = ui.allocate_rect(rect, egui::Sense::click_and_drag());

                        if let Some(texture) = &self.image_texture[i] {
                            ui.painter().image(texture.id(), rect, Rect::from_min_max(pos2(0.0, 0.0), pos2(1.0, 1.0)), Color32::WHITE);
                        }
                        if self.response[i][r].drag_started() {
                            self.selection = None;
                        }
                        if self.response[i][r].dragged() {
                            self.pos[i][r] += self.response[i][r].drag_delta();
                        }
                        if self.response[i][r].clicked() {
                            match i {
                                0 => self.selection = Some(Item::Shrimp),
                                1 => self.selection = Some(Item::Xiaolongbao),
                                2 => self.selection = Some(Item::ChickDumpling),
                                3 => self.selection = Some(Item::FriedBun),
                                _ => {}
                            };
                        }
                        if self.response[i][r].drag_stopped() {
                            match i {
                                0 => self.selection = Some(Item::Shrimp),
                                1 => self.selection = Some(Item::Xiaolongbao),
                                2 => self.selection = Some(Item::ChickDumpling),
                                3 => self.selection = Some(Item::FriedBun),
                                _ => {}
                            };
        
                        }
                        
                    /*     ui.label(
                            "X: ".into()+ &self.pos[i][r].x.to_string() + " Y: " + &self.pos[i][r].y.to_string(),
                        );  */
                        
                  /*       if let Some(start_time) = self.timer_start[r] {
                            let elapsed = start_time.elapsed();
                            let min: i32 = (elapsed.as_secs_f32() / 60.0) as i32;
                            ui.label(format!("Timer: {}m {:.0} s", min, elapsed.as_secs_f32() % 60.0));
                        } */
                    }
                }
            }
        });

        egui::TopBottomPanel::bottom("bot").show(ctx, |ui| {});

        ctx.request_repaint();
    }
}

pub fn timenow() -> String {
    let time: DateTime<Local> = Local::now();
    let now = time.format("%H:%M").to_string();
    now
}