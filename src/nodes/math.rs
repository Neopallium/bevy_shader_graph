use glam::{Mat2, Mat3, Mat4, Vec2, Vec3, Vec4};

use anyhow::Result;

use node_engine::*;

#[macro_export]
macro_rules! impl_math_node {
  ( $mod_name:ident, $ty_name:ident, $v:ident, $name:expr ) => {
    impl_math_node!($mod_name, $ty_name, $v, $v, $v, $name);
  };
  ( $mod_name:ident, $ty_name:ident, $a:ident, $b:ident, $out:ident, $name:expr ) => {
    impl_node! {
      mod $mod_name {
        NodeInfo {
          name: $name,
          description: "Simple math node",
          category: ["Math", "Basic"],
        }

        pub enum Op {
          Add,
          Sub,
          Mul,
          Div,
        }

        /// Docs.
        #[derive(Default)]
        pub struct $ty_name {
          /// Input `A`.
          pub a: Input<$a>,
          /// Input `B`.
          pub b: Input<$b>,
          /// Math operation `op`.
          pub op: Param<Op>,
          /// Output `out`.
          pub out: Output<$out>,
        }

        impl $ty_name {
          pub fn new() -> Self {
            Default::default()
          }
        }

        impl NodeImpl for $ty_name {
          fn eval(
            &self,
            graph: &NodeGraph,
            execution: &mut NodeGraphExecution,
            _id: NodeId,
          ) -> Result<Value> {
            let a = self.a.eval(graph, execution)?;
            let b = self.b.eval(graph, execution)?;
            match self.op {
              Op::Add => Ok((a + b).to_value()),
              Op::Sub => Ok((a - b).to_value()),
              Op::Mul => Ok((a * b).to_value()),
              Op::Div => Ok((a / b).to_value()),
            }
          }

          fn compile(
            &self,
            graph: &NodeGraph,
            compile: &mut NodeGraphCompile,
            id: NodeId,
          ) -> Result<()> {
            let a = self.a.compile(graph, compile)?;
            let b = self.b.compile(graph, compile)?;
            let code = match self.op {
              Op::Add => format!("{a} + {b}"),
              Op::Sub => format!("{a} - {b}"),
              Op::Mul => format!("{a} * {b}"),
              Op::Div => format!("{a} / {b}"),
            };
            let block = compile.current_block()?;
            block.append_output(id, code);
            Ok(())
          }
        }
      }
    }
  };
}

impl_math_node!(scalar_f32, ScalarMath, f32, "Scalar Math");
impl_math_node!(vec2, Vec2Math, Vec2, "Vec2 Math");
impl_math_node!(vec3, Vec3Math, Vec3, "Vec3 Math");
impl_math_node!(vec4, Vec4Math, Vec4, "Vec4 Math");

impl_node! {
  mod fract_vec4 {
    NodeInfo {
      name: "Fraction Vec4",
      description: "",
      category: ["Math"],
    }

    /// Fraction node.
    #[derive(Default)]
    pub struct FractionNode {
      /// Input
      pub input: Input<Vec4>,
      /// Output
      pub output: Output<Vec4>,
    }

    impl FractionNode {
      pub fn new() -> Self {
        Default::default()
      }
    }

    impl NodeImpl for FractionNode {
      fn compile(&self, graph: &NodeGraph, compile: &mut NodeGraphCompile, id: NodeId) -> Result<()> {
        let input = self.input.compile(graph, compile)?;
        let block = compile.current_block()?;
        block.append_output(id, format!("fract({input})"));
        Ok(())
      }
    }
  }
}

#[macro_export]
macro_rules! impl_mat_math_node {
  ( $mod_name:ident, $ty_name:ident, $v:ident, $name:expr ) => {
    impl_mat_math_node!($mod_name, $ty_name, $v, $v, $v, $name);
  };
  ( $mod_name:ident, $ty_name:ident, $a:ident, $b:ident, $out:ident, $name:expr ) => {
    impl_node! {
      mod $mod_name {
        NodeInfo {
          name: $name,
          description: "Simple math node",
          category: ["Math", "Basic"],
        }

        pub enum Op {
          Add,
          Sub,
          Mul,
        }

        /// Docs.
        #[derive(Default)]
        pub struct $ty_name {
          /// Input `A`.
          pub a: Input<$a>,
          /// Input `B`.
          pub b: Input<$b>,
          /// Math operation `op`.
          pub op: Param<Op>,
          /// Output `out`.
          pub out: Output<$out>,
        }

        impl $ty_name {
          pub fn new() -> Self {
            Default::default()
          }
        }

        impl NodeImpl for $ty_name {
          fn eval(
            &self,
            graph: &NodeGraph,
            execution: &mut NodeGraphExecution,
            _id: NodeId,
          ) -> Result<Value> {
            let a = self.a.eval(graph, execution)?;
            let b = self.b.eval(graph, execution)?;
            match self.op {
              Op::Add => Ok((a + b).to_value()),
              Op::Sub => Ok((a - b).to_value()),
              Op::Mul => Ok((a * b).to_value()),
            }
          }

          fn compile(
            &self,
            graph: &NodeGraph,
            compile: &mut NodeGraphCompile,
            id: NodeId,
          ) -> Result<()> {
            let a = self.a.compile(graph, compile)?;
            let b = self.b.compile(graph, compile)?;
            let code = match self.op {
              Op::Add => format!("{a} + {b}"),
              Op::Sub => format!("{a} - {b}"),
              Op::Mul => format!("{a} * {b}"),
            };
            let block = compile.current_block()?;
            block.append_output(id, code);
            Ok(())
          }
        }
      }
    }
  };
}

impl_mat_math_node!(mat2, Mat2Math, Mat2, "Mat2 Math");
impl_mat_math_node!(mat3, Mat3Math, Mat3, "Mat3 Math");
impl_mat_math_node!(mat4, Mat4Math, Mat4, "Mat4 Math");
