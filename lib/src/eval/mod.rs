use hyperon_atom::*;
use hyperon_atom::matcher::*;

pub struct State {
    atom: Atom,
    bindings: Bindings,
}

pub trait GroundedAtomType {
    fn as_atom(&self) -> &Atom;
    fn equal<T>(a: &T, b: &T) -> bool where Self: Sized, T: GroundedAtomType;
    fn new_item(&mut self, text: &str) -> Atom;
}

pub trait Channel {
    fn send(&mut self, state: State);
}

pub trait Evaluator {
    fn eval(&mut self, input: State, output: &mut dyn Channel);
}

pub trait Module {
    fn init(&mut self, output: &mut dyn Channel);
    fn get_name(&self) -> String;
    fn set_index(&mut self, index: ModuleIndexRef);
    fn evaluator(&mut self) -> &mut dyn Evaluator;
}

pub struct ModuleIndexRef(std::rc::Rc<std::cell::RefCell<ModuleIndex>>);

pub struct ModuleIndex {
}

pub struct ModuleId(usize);

impl ModuleIndex {
    pub fn add_module<M>(&mut self, module: M) -> ModuleId where M: Module {
        todo!();
    }

    pub fn mod_by_func(&self, func: &Atom) -> ModuleId {
        todo!();
    }

    pub fn module(&self, mod_id: &ModuleId) -> &dyn Module {
        todo!();
    }

    pub fn add_function(&mut self, func: &Atom, mod_id: ModuleId) {
        todo!();
    }
    pub fn remove_function(&mut self, func: &Atom) {
        todo!();
    }

    pub fn add_type<T>(&mut self, typ: T, mod_id: ModuleId) where T: GroundedAtomType {
        todo!();
    }
    pub fn remove_type(&mut self, typ: &Atom) {
        todo!();
    }
}

struct Metta<M: Module> {
    module: M,
    mod_index: ModuleIndexRef,
}

impl<M: Module> Metta<M> {
    fn eval(&mut self, input: &Atom, output: &mut dyn Channel) {
    }
}
