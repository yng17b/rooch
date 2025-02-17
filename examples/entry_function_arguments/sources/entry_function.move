module rooch_examples::entry_function {
   use moveos_std::event;
   use moveos_std::object_id::ObjectID;
   use moveos_std::storage_context::StorageContext;

   struct BoolEvent {
      value: bool
   }
   public entry fun emit_bool(ctx: &mut StorageContext, value: bool) {
      event::emit<BoolEvent>(ctx, BoolEvent { value });
   }

   struct U8Event {
        value: u8
   }
   public entry fun emit_u8(ctx: &mut StorageContext, value: u8) {
      event::emit<U8Event>(ctx, U8Event { value });
   }

   struct U16Event {
      value: u16
   }
   public entry fun emit_u16(ctx: &mut StorageContext, value: u16) {
      event::emit<U16Event>(ctx, U16Event { value });
   }

   struct U32Event {
      value: u32
   }
   public entry fun emit_u32(ctx: &mut StorageContext, value: u32) {
      event::emit<U32Event>(ctx, U32Event { value });
   }

   struct U64Event {
      value: u64
   }
   public entry fun emit_u64(ctx: &mut StorageContext, value: u64) {
      event::emit<U64Event>(ctx, U64Event { value });
   }

   struct U128Event {
      value: u128
   }
   public entry fun emit_u128(ctx: &mut StorageContext, value: u128) {
      event::emit<U128Event>(ctx, U128Event { value });
   }

   struct U256Event {
      value: u256
   }
   public entry fun emit_u256(ctx: &mut StorageContext, value: u256) {
      event::emit<U256Event>(ctx, U256Event { value });
   }

   struct AddressEvent {
      value: address
   }
   public entry fun emit_address(ctx: &mut StorageContext, value: address) {
      event::emit<AddressEvent>(ctx, AddressEvent { value });
   }

   struct ObjectIdEvent {
      value: ObjectID
   }
   public entry fun emit_object_id(ctx: &mut StorageContext, value: ObjectID) {
      event::emit<ObjectIdEvent>(ctx, ObjectIdEvent { value });
   }

   struct StringEvent {
      value: std::string::String
   }
   public entry fun emit_string(ctx: &mut StorageContext, value: std::string::String) {
      event::emit<StringEvent>(ctx, StringEvent { value });
   }

   struct VecU8Event {
      value: vector<u8>
   }
   public entry fun emit_vec_u8(ctx: &mut StorageContext, value: vector<u8>) {
      event::emit<VecU8Event>(ctx, VecU8Event { value });
   }

   struct VecObjectIDEvent {
      value: vector<ObjectID>
   }
   
   public entry fun emit_vec_object_id(ctx: &mut StorageContext, value: vector<ObjectID>) {
      event::emit<VecObjectIDEvent>(ctx, VecObjectIDEvent { value });
   }

   public entry fun emit_mix(ctx: &mut StorageContext, value1: u8, value2: vector<ObjectID>) {
      event::emit<U8Event>(ctx, U8Event { value: value1 });
      event::emit<VecObjectIDEvent>(ctx, VecObjectIDEvent { value: value2 });     
   }
}