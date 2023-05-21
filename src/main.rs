use eframe::egui;

const W_WITH:f32=600.0;
const W_HEIGTH:f32=500.0;

fn main() ->Result<(), eframe::Error>{
    let native_options=eframe::NativeOptions{
        initial_window_size:Some(egui::vec2(W_WITH, W_HEIGTH)),
        resizable:false,
        max_window_size:Some(egui::vec2(W_WITH, W_HEIGTH)),
        ..Default::default()
    };

    eframe::run_native(
        "TODO APP",
        native_options,
        Box::new(|_cc|Box::<Todo>::default()),
    )

}


struct Todo{
    text:String,
    text_new:Vec<String>,
    delete:bool,
}


impl Default for Todo{
    fn default()-> Self{
        Self{
            text:"my todo".to_owned(),
            text_new:Vec::new(),
            delete:false,

        }
    
    }
}


impl eframe::App for Todo{
    fn update(&mut self, ctx: &egui::Context, _frame:&mut eframe::Frame){
       egui::CentralPanel::default().show(ctx,|ui|{

        ui.horizontal(|ui|{
            ui.text_edit_singleline(&mut self.text);

            if ui.button("add").clicked(){
                self.text_new.push(self.text.to_owned());
                println!("{:?}", self.text_new)
            }
       }); 

       ui.separator();

       ui.heading("Your Todo's");

       let mut todo_len=self.text_new.len();

       egui::ScrollArea::vertical().show(ui,|ui|{

            for t in 0..todo_len{
        
                ui.horizontal(|ui|{
            
                    ui.label(format!("{}", self.text_new[t as usize]));

                    if ui.button("X").clicked(){
                        println!("delete :{}",t);
                        self.text_new.remove(t);
                        println!("{:?}", self.text_new);
                        todo_len=self.text_new.len();
                        println!("the length is {todo_len}");
                    }

                });
                ui.separator();
            }
       });


    });

    }
}