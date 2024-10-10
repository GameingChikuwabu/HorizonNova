struct ComponentInfo{
    pub name : String,
    pub fields : Vec<FieldInfo>,
    pub id : u32,
    pub size : usize,
    pub alignment : usize,
    pub type_id : TypeId,
}

struct FieldInfo{
    pub name : String,
    pub ty : TypeId,
    pub offset : usize,
}