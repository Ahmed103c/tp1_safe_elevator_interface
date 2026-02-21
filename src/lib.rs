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

#[derive(Debug, Clone, PartialEq)]
pub struct ElevatorStatus {
    pub floor: i32,
    pub state: State,
    pub queue: Vec<i32>,
}


#[derive(Debug, PartialEq)] 
pub enum ElevatorError {
    InvalidFloor(i32),
    CannotMoveDoorsOpen,
    CannotOpenWhileMoving,
    DoorsAlreadyClosed,
    EmptyQueue,
}

impl  Elevator {
    pub fn new(start_floor:i32) -> Result<Elevator,ElevatorError>{
        if (start_floor < 0) | (start_floor > 5) {
            return Err(ElevatorError::InvalidFloor(start_floor));
        }
        Ok(Elevator { current_floor: start_floor, state: State::Idle, queue: Vec::new() })
    }

    pub fn call(&mut self, client_floor: i32) -> Result<(), ElevatorError> {
        if client_floor < 0 || client_floor > 5 {
            return Err(ElevatorError::InvalidFloor(client_floor));
        }

        if client_floor == self.current_floor || self.queue.contains(&client_floor) {
            return Ok(());
        }
        self.queue.push(client_floor);
        

        if self.state == State::Idle {
            if client_floor > self.current_floor {
                self.state = State::MovingUp;
            } else {
                self.state = State::MovingDown;
            }
        }
        Ok(())
    }

    pub fn step(&mut self) -> Result<(),ElevatorError>{
        match self.state {
            State::MovingUp => {
                if let Some(&destination) = self.queue.first() {
                    self.current_floor += 1;  
                    
                    if self.current_floor == destination {  
                        self.state = State::DoorsOpen;
                        self.queue.remove(0);
                    }
                }
            }

            State::MovingDown => {
                if let Some(&destination) = self.queue.first() {
                    self.current_floor -= 1;  
                    
                    if self.current_floor == destination {  
                        self.state = State::DoorsOpen;
                        self.queue.remove(0);
                    }
                }
            }
            State::DoorsOpen => {
                return Err(ElevatorError::CannotMoveDoorsOpen);
            }
            State::Idle => {
                if self.queue.is_empty() {
                    return Err(ElevatorError::EmptyQueue);
                }
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
    pub fn queue(&self) -> Vec<i32>{
        self.queue.clone()
    }

    pub fn open_doors(&mut self) -> Result<(), ElevatorError> {
        if matches!(self.state, State::MovingUp | State::MovingDown | State::DoorsOpen){
            return Err(ElevatorError::CannotOpenWhileMoving);
        } 
        self.state = State::DoorsOpen;
        Ok(())
    }   

    pub fn close_doors(&mut self) -> Result<(), ElevatorError> {
        if self.state != State::DoorsOpen {
            return Err(ElevatorError::DoorsAlreadyClosed);
        }
        // Si queue non vide, déterminer la direction
        if let Some(&next) = self.queue.first() {
            if next > self.current_floor {
                self.state = State::MovingUp;
            } else {
                self.state = State::MovingDown;
            }
        } else {
            self.state = State::Idle;
        }
        Ok(())
    }

    pub fn status(&self) -> ElevatorStatus{
        ElevatorStatus { floor: self.current_floor, state: self.state , queue: self.queue() }
    } 
}

