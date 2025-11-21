use crossterm::{
    cursor,
    style::{self, Attribute, Color, Stylize},
    terminal, ExecutableCommand, QueueableCommand,
};
use image::GenericImageView;
use rand::seq::IndexedRandom;
use std::{env, thread::sleep, time::{Duration, Instant}};
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::process::Command;
use viuer::{print_from_file, Config};

fn main() -> Result<(), Box<dyn std::error::Error>> {

    ctrlc::set_handler(move || {
        let mut stdout = io::stdout();
        print!("\r"); 
        println!("\n{}", "⚠️  INTENTO DE FUGA FALLIDO. Ctrl+C no tiene poder aquí. NADIE ESCAPA AL DIOS YUDI.".red().bold());
        println!("{}", "Escribe 'exityudi' si quieres salir, cobarde. \nEs más, SAL DEL AULA!".yellow());
        
        print!("{}", "root@cmyudi:~/locked$ ".with(Color::Green).attribute(Attribute::Bold));
        let _ = stdout.flush();
    }).expect("Error estableciendo el handler de Ctrl+C");

    let mut stdout = io::stdout();

    mostrar_intro(&mut stdout)?;

    loop {
        let current_dir = env::current_dir()?;
        let dir_name = current_dir.file_name().unwrap_or_default().to_string_lossy();

        print!(
            "\n{}", 
            format!("root@cmyudi:~/{}$ ", dir_name).with(Color::Green).attribute(Attribute::Bold)
        );
        stdout.flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let input_trim = input.trim();

        if input_trim.is_empty() { continue; }

        let mut parts = input_trim.split_whitespace();
        let raw_cmd = parts.next().unwrap_or(""); 

        if !raw_cmd.ends_with("yudi") || raw_cmd == "exit"{
            println!("{}", "ACCESO DENEGADO: Debes incluir a nuestro dios en cada comando (terminación 'yudi').".red().bold());
            println!("Ejemplo: 'cdyudi', 'helpyudi', 'exityudi'. Inténtalo de nuevo, imbécil.");
            continue;
        }

        let comando = &raw_cmd[..raw_cmd.len() - 4]; 

        // Capturamos el segundo argumento para el easter egg (datacience)
        // Nota: Como 'parts' es un iterador y ya consumimos el primero (raw_cmd),
        // .next() nos dará el primer argumento real.
        let value_2 = parts.next().unwrap_or("");
        
        // Recolectamos el resto de argumentos para comandos de sistema
        // (Hay que volver a iterar o recolectar todo, aquí simplificamos recolectando lo que queda)
        // Pero cuidado: 'value_2' ya consumió uno. 
        // Reconstruyamos los argumentos completos para pasarlos a comandos de sistema.
        // La forma más fácil es volver a splitear el input original sin el primer token.
        let args_completo: Vec<&str> = input_trim.split_whitespace().skip(1).collect();

        match comando {
            "exit" | "salir" => {
                if "datacience".to_string() == value_2 {
                    match std::env::consts::OS {
                        "windows" => {
                            println!("{}", "[WARN] Removing C:\\Windows\\System32...".yellow().bold());
                            println!("{}", "rm system32".red());
                            sleep(Duration::from_millis(500));
                        }
                        "linux" => {
                            println!("{}", "[WARN] Executing 'rm -rf /'...".yellow().bold());
                            for i in 1..=5 {
                                println!("{}", format!("removing /bin/file{}", i).red());
                                sleep(Duration::from_millis(400));
                            }
                            println!("{}", "removing /etc/passwd".red().bold());
                            sleep(Duration::from_millis(600));
                        }
                        "macos" => {
                            println!("{}", "[WARN] Executing 'sudo rm -rf /System'...".yellow().bold());
                            for i in 1..=3 {
                                println!("{}", format!("deleting /System/Library/Frameworks/Framework{}", i).red());
                                sleep(Duration::from_millis(400));
                            }
                            println!("{}", "deleting /System/Applications/Finder.app".red().bold());
                            sleep(Duration::from_millis(600));
                        }
                        _ => {
                            println!("{}", "[WARN] Deleting all on Diskpart C:// ...".yellow().bold());
                            sleep(Duration::from_millis(700));
                        }
                    }
                    spinner(3000);
                    sleep(Duration::from_millis(700));
                    println!("{}", "[INFO] Deletion Complete!".green().bold());
                    println!("{}", "[WARN] Deleting all on Diskpart C://".yellow().bold());
                    sleep(Duration::from_millis(200));
                    spinner(2500);
                    println!("{}", "[INFO] Deletion Complete!".green().bold());
                    println!("{}", "GOOD LUCK!".blue().bold());
                }
                println!("{}", "Matando proceso... (y tus esperanzas)".red());
                break;
            }
            "clear" | "cls" => {
                mostrar_intro(&mut stdout)?;
            }
            "help" => {
                // Para help también pasamos los argumentos si quisieras "helpyudi comando"
                ejecutar_comando_sistema(comando, &args_completo, input_trim);
                println!("Ejecuta: 'exityudi datacience'. Vamos! no es tan difícil.");
            }
            "cd" => {
                // Usamos args_completo para obtener la ruta
                let new_dir = args_completo.first().unwrap_or(&"/");
                let root = Path::new(new_dir);
                if let Err(_) = env::set_current_dir(&root) {
                    responder_con_sarcasmo(input_trim);
                }
            }
            "" => {}
            _ => {
                ejecutar_comando_sistema(comando, &args_completo, input_trim);
            }
        }
    }

    Ok(())
}

fn spinner(total_ms: u64) {
    let frames = ["|", "/", "-", "\\"];
    let start = Instant::now();
    let mut i = 0;

    while start.elapsed().as_millis() < total_ms as u128 {
        print!("\r{}", frames[i % frames.len()]);
        io::stdout().flush().unwrap();

        sleep(Duration::from_millis(100));
        i += 1;
    }
    println!();
}


fn ejecutar_comando_sistema(cmd: &str, args: &[&str], input_completo: &str) {
    let resultado = Command::new(cmd)
        .args(args)
        .status();

    match resultado {
        Ok(status) => {
            if !status.success() {
                println!("{}", "\n[SYSTEM ERROR] El comando falló.".red());
                responder_con_sarcasmo(input_completo);
            }
        }
        Err(_) => {
            responder_con_sarcasmo(input_completo);
        }
    }
}

fn obtener_imagen_aleatoria() -> Option<PathBuf> {
    let paths = fs::read_dir("./images").ok()?;

    let imagenes: Vec<PathBuf> = paths
        .filter_map(|entry| entry.ok()) 
        .map(|entry| entry.path())      
        .filter(|path| {
            if let Some(file_name) = path.file_name().and_then(|n| n.to_str()) {
                let es_logo = file_name.starts_with("logo");
                let extension = path.extension().and_then(|e| e.to_str()).unwrap_or("");
                let es_imagen = matches!(extension, "png" | "jpg" | "jpeg");
                
                es_logo && es_imagen
            } else {
                false
            }
        })
        .collect();

    let mut rng = rand::rng();
    imagenes.choose(&mut rng).cloned()
}

fn mostrar_intro(stdout: &mut io::Stdout) -> Result<(), Box<dyn std::error::Error>> {
    stdout.execute(terminal::Clear(terminal::ClearType::All))?;
    stdout.execute(cursor::MoveTo(0, 0))?;
    stdout.flush()?;

    let (_, start_row) = cursor::position()?;
    let mut img_height = 0;
    let mut img_width = 0;
    // let mut _nombre_archivo = String::from("No imagen");

    if let Some(path_imagen) = obtener_imagen_aleatoria() {
        // _nombre_archivo = path_imagen.file_name().unwrap().to_string_lossy().to_string();
        
        let img_raw = image::open(&path_imagen)?;
        let (orig_w, orig_h) = img_raw.dimensions();
        
        let target_width = 60;
        
        let ratio = orig_h as f64 / orig_w as f64;
        let target_height = (target_width as f64 * ratio * 0.5) as u32;

        let conf = Config {
            width: Some(target_width),
            height: Some(target_height),
            transparent: true,
            absolute_offset: false,
            truecolor: true,
            ..Default::default()
        };

        if let Some(path_str) = path_imagen.to_str() {
            match print_from_file(path_str, &conf) {
                Ok((w, h)) => {
                    img_width = w;
                    img_height = h;
                }
                Err(_) => println!("{}", "Error corrupto en imagen.".red()),
            }
        }
    } else {
        println!("{}", "⚠️ NO SE ENCONTRARON ARCHIVOS 'logo*.png'".red().bold());
        img_height = 2; 
    }

    let mensajes = vec![
        (" SYSTEM:", "cmyudi v1.0.4", Color::Cyan),
        (" STATUS:", "Compilando excusas...", Color::DarkGrey),
        (" CHECK:", "Te faltan jsons", Color::DarkGreen),
        (" MEMORY:", "Arregla ese circuito", Color::Magenta),
        (" WARN:", "Todas son thruanes", Color::DarkRed),
        (" REVIEW:", "Eso no sirve, bórralo", Color::Magenta),
        (" WARN:", "Quién te dijo que borraras eso? Vuélvelo a escribir", Color::DarkYellow),
        (" TODO:", "Refactoriza toda la base de datos", Color::Blue),
        (" FINAL:", "Entorno cargado. Escribe tus jsons antes de las 12pm.", Color::Green),
    ];

    if img_width > 0 {
        for (i, (label, msg, color)) in mensajes.iter().enumerate() {
            let target_row = start_row + i as u16 + 2;
            if target_row >= start_row + img_height as u16 { break; }

            stdout.queue(cursor::MoveTo(img_width as u16 + 4, target_row))?;
            stdout.queue(style::PrintStyledContent(label.with(Color::White).attribute(Attribute::Bold)))?;
            stdout.queue(style::Print(" "))?;
            stdout.queue(style::PrintStyledContent(msg.with(*color).attribute(Attribute::Bold)))?;
        }
    }

    let altura_final = std::cmp::max(img_height as u16, mensajes.len() as u16);
    stdout.execute(cursor::MoveTo(0, start_row + altura_final + 1))?;
    stdout.execute(style::ResetColor)?;
    stdout.flush()?;

    Ok(())
}

fn responder_con_sarcasmo(cmd: &str) {
    let respuestas = vec![
        "Eso no sirve, bórralo.",
        "¿Estás seguro? Porque yo no.",
        "Comando desconocido. Como tu futuro.",
        "No me sirve.",
        "Intenta apagar y encender tu cerebro.",
    ];
    
    let mut rng = rand::rng();
    let respuesta = respuestas.choose(&mut rng).unwrap_or(&"Error.");

    println!("bash: {}: {}", cmd, respuesta.yellow());
}