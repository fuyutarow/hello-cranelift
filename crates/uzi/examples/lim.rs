use cranelift::codegen::{isa, settings};
use cranelift::frontend::{FunctionBuilder, FunctionBuilderContext};
use cranelift::prelude::*;
use cranelift_module::{Linkage, Module};
use cranelift_simplejit::{SimpleJITBackend, SimpleJITBuilder};
use target_lexicon::Triple;

fn main() {
    // Craneliftの設定とターゲットISAの作成。
    let isa_builder = isa::lookup(Triple::host()).unwrap();
    let flag_builder = settings::builder();
    let isa = isa_builder
        .finish(settings::Flags::new(flag_builder))
        .unwrap();

    // JITモジュールの作成。
    let builder = SimpleJITBuilder::new(cranelift_module::default_libcall_names());
    let mut module = Module::new(builder);

    // 関数シグネチャの作成。
    let mut ctx = module.make_context();
    ctx.func.signature.returns.push(AbiParam::new(types::I32));

    // 関数ビルダーコンテキストの作成。
    let mut fn_ctx = FunctionBuilderContext::new();
    let mut builder = FunctionBuilder::new(&mut ctx.func, &mut fn_ctx);

    // 関数のエントリーブロックを作成し、そのブロック内での命令を追加。
    let block = builder.create_block();
    builder.switch_to_block(block);
    builder.seal_block(block);
    let value = builder.ins().iconst(types::I32, 42);
    builder.ins().return_(&[value]);

    // 関数の完成。
    builder.finalize();

    // JITモジュールに関数を追加し、バイナリを生成。
    let id = module
        .declare_function("test", Linkage::Export, &ctx.func.signature)
        .unwrap();
    module.define_function(id, &mut ctx).unwrap();
    module.clear_context(&mut ctx);
    module.finalize_definitions();

    // 関数へのポインタを取得し、実行。
    let code = module.get_finalized_function(id);
    let code_fn = unsafe { std::mem::transmute::<_, fn() -> i32>(code) };
    println!("Returned: {}", code_fn());
}
