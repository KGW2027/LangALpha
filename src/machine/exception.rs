use std::process;

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

pub fn throw_exception(ex: Exception, ptr: u64) {
    println!("Error : {} on {}", ex.get_message(), ptr);
    process::exit(1);
}