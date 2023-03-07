
pub enum ExceptionType {
    EOFException,
    AssertionFailed,
}


pub struct Exception {
    except: ExceptionType,
    message: String
}

impl Exception {
    pub fn new(class: ExceptionType, msg: String) -> Exception {
        Exception {
            except:class, message:msg
        }
    }

    pub fn get_message(&self) -> String {
        self.message.clone()
    }
}