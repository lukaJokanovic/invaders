use std::{time::Duration, cmp::max};

use rusty_time::Timer;

use crate::{NUM_COLS, frame::Drawable};

pub struct Invader{
    x: usize,
    y: usize,
}

pub struct Invaders{
    army: Vec<Invader>,
    move_timer: Timer,
    direction: i32,
}

impl Invaders {
    pub fn new() ->Self{
        let mut army = Vec::new();
        for x in 2..(NUM_COLS - 2){
            for y in 1..9{
               if x % 2 == 0 && y % 2 == 0 {
                army.push(Invader { x, y });
               }
            }
        }
        Self { 
            army,
            move_timer: Timer::from_millis(2000),
            direction: 1,
        }
    }

    pub fn update(&mut self, delta: Duration) -> bool{
        self.move_timer.update(delta);
        if self.move_timer.ready{
            self.move_timer.reset();
            let mut move_downwards = false;
            if self.direction == -1 {
                let min_x = self.army.iter().fold(usize::MAX, |acc, val| if acc > val.x {val.x} else {acc});
                if min_x == 0 {
                    self.direction = 1;
                    move_downwards = true;
                }
            } else{
                let max_x = self.army.iter().fold(0, |acc, val| if acc < val.x {val.x} else {acc});
                if max_x == NUM_COLS - 1 {
                    self.direction = -1;
                    move_downwards = true;
                }
            }
            if move_downwards {
                let new_duration = max(self.move_timer.duration.as_millis() - 250, 250);
                self.move_timer = Timer::from_millis(new_duration as u64);
                self.army.iter_mut().for_each(|invader| invader.y += 1);
            }else{
                self.army.iter_mut().for_each(|invader| invader.x = (invader.x as i32 + self.direction) as usize);

            }

            true
        } else {
            false
        }
    }
}

impl Drawable for Invader{
    fn draw(&self, frame: &mut crate::frame::Frame) {
        frame[self.x][self.y] = "⍒";
    }
}

impl Drawable for Invaders{
    fn draw(&self, frame: &mut crate::frame::Frame) {
        self.army.iter().for_each(|invader| invader.draw(frame));
    }
}