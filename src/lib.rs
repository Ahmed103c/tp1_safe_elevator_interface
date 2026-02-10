#[derive(Debug, PartialEq, Clone, Copy)]
pub enum State {
    Idle,
    MovingUp,
    MovingDown,
    DoorsOpen,
}

pub struct Elevator {
    current_floor: i32,
    state: State,
    queue: Vec<i32>,
}

#[derive(Debug)]
pub enum ElevatorError {
    InvalidFloor

}

impl  Elevator {
    pub fn new(start_floor:i32) -> Result<Elevator,ElevatorError>{
        if start_floor < 0{
            return Err(ElevatorError::InvalidFloor);
        }
        Ok(Elevator { current_floor: start_floor, state: State::Idle, queue: Vec::new() })
    }

    pub fn call(&mut self, client_floor:i32) -> Result<(),ElevatorError>{
        if client_floor < 0{
            return Err(ElevatorError::InvalidFloor);
        }
        if client_floor > self.current_floor {
            self.state = State::MovingUp;
        } else if client_floor < self.current_floor {
            self.state = State::MovingDown;
        }

        self.queue.push(client_floor);
        Ok(())
    }

    pub fn step(&mut self) -> Result<(),ElevatorError>{
        match self.state {
            State::MovingUp => {
                if let Some(&destination) = self.queue.first() {
                    self.current_floor += 1;  // Monte d'abord
                    
                    if self.current_floor == destination {  // Puis vérifie
                        self.state = State::DoorsOpen;
                        self.queue.remove(0);
                    }
                }
            }

            State::MovingDown => {
                if let Some(&destination) = self.queue.first() {
                    self.current_floor -= 1;  // Descend d'abord
                    
                    if self.current_floor == destination {  // Puis vérifie
                        self.state = State::DoorsOpen;
                        self.queue.remove(0);
                    }
                }
            }
            State::Idle | State::DoorsOpen => {

            }
        }
        Ok(())
    }

    pub fn floor(&self) -> i32{
        self.current_floor
    }
    pub fn state(&self) -> State{
        self.state
    }
    pub fn queue(&self) -> &Vec<i32>{
        &self.queue
    }

}