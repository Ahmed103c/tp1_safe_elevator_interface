#[derive(Debug, PartialEq, Clone, Copy)]
pub enum State {
    Idle,
    MovingUp,
    MovingDown,
    DoorsOpen,
}

#[derive(Debug)]
pub struct Elevator {
    current_floor: i32,
    state: State,
    queue: Vec<i32>,
}

#[derive(Debug, PartialEq)] 
pub enum ElevatorError {
    InvalidFloor(i32),
    CannotMoveDoorsOpen
}

impl  Elevator {
    pub fn new(start_floor:i32) -> Result<Elevator,ElevatorError>{
        if (start_floor < 0) | (start_floor > 5) {
            return Err(ElevatorError::InvalidFloor(start_floor));
        }
        Ok(Elevator { current_floor: start_floor, state: State::Idle, queue: Vec::new() })
    }

    pub fn call(&mut self, client_floor:i32) -> Result<(),ElevatorError>{
        if (client_floor < 0) | (client_floor > 5) {
            return Err(ElevatorError::InvalidFloor(client_floor));
        }
        else if client_floor > self.current_floor {
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
            State::DoorsOpen => {
                return Err(ElevatorError::CannotMoveDoorsOpen);
            }
            State::Idle => {

            }
        }
        Ok(())
    }

    pub fn floor(&self) -> i32{
        self.current_floor
    }
    //L'importance de & dans les tests 
    pub fn state(&self) -> State{
        self.state
    }
    pub fn queue(&self) -> &Vec<i32>{
        &self.queue
    }

    pub fn open_doors(&mut self) -> Result<(), ElevatorError> {
        self.state = State::DoorsOpen;
        Ok(())
    }   
}

