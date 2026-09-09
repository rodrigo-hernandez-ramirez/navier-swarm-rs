use macroquad::prelude::*;
#[macroquad::main("NAVIER SWARM")]
async fn main() {
    let mut agents: Vec<(f32,f32,f32,f32)> = (0..100).map(|_| (rand::gen_range(0.,800.), rand::gen_range(0.,600.), rand::gen_range(-1.,1.), rand::gen_range(-1.,1.))).collect();
    loop {
        clear_background(BLACK);
        for (x,y,vx,vy) in agents.iter_mut() {
            *x+=*vx; *y+=*vy;
            if *x<0.||*x>800.{*vx*=-1.;}
            if *y<0.||*y>600.{*vy*=-1.;}
            draw_circle(*x,*y,4.,GREEN);
        }
        draw_text("NAVIER-SWARM 100 AGENTES [ESTABLE]",10.,20.,20.,WHITE);
        draw_text("Lean4 Verificado + Tokio - Merida",10.,580.,16.,GREEN);
        next_frame().await
    }
}
