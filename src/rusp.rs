// Self-Contained Rusp Data //
//
// This file should be built so that it can be copied into a mod {} in a file
// and included in compiled rusp modules, or included in a project as normal.
// It will require being setup in the right order so that there are no dependency
// issues.
//
// We want to keep this as simple as possible and only contain the data structures
// and all they need to work. It should contain the absolute core of the language
// and any additions, like extra functions, should be in a separate file called
// stdlib or something. The stdlib can include this and also be included as a
// file or have it's contents copied into a mod in a compiled rusp file.

// Data traits ////////////////////////////////////////////////////////////////

pub trait DisplayRep {
    fn to_display(&self) -> String;
}

pub trait ExternalRep {
    fn to_external(&self) -> String;
}

pub trait ValueIterator {
    fn values(&self) -> Box<dyn Iterator<Item = Val>>;
}
