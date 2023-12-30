#import bevy_pbr::{
	pbr_fragment::pbr_input_from_standard_material,
	pbr_functions::alpha_discard,
}

#ifdef PREPASS_PIPELINE
#import bevy_pbr::{
	prepass_io::{VertexOutput, FragmentOutput},
	pbr_deferred_functions::deferred_output,
}
#else
#import bevy_pbr::{
	forward_io::{VertexOutput, FragmentOutput},
	pbr_functions::{apply_pbr_lighting, main_pass_post_lighting_processing},
	pbr_types::STANDARD_MATERIAL_FLAGS_UNLIT_BIT,
}
#endif

@fragment
fn fragment(
  in: VertexOutput,
  @builtin(front_facing) is_front: bool,
) -> FragmentOutput {
	// get PbrInput from StandardMaterial bindings.
	var pbr_input = pbr_input_from_standard_material(in, is_front);

	// alpha discard
  pbr_input.material.base_color = alpha_discard(pbr_input.material, pbr_input.material.base_color);

#ifdef PREPASS_PIPELINE
	// No lighting in deferred mode.
	let out = deferred_output(in, pbr_input);
#else
	var out: FragmentOutput;
  if (pbr_input.material.flags & STANDARD_MATERIAL_FLAGS_UNLIT_BIT) == 0u {
		out.color = apply_pbr_lighting(pbr_input);
	} else {
		out.color = pbr_input.material.base_color;
	}

	// Apply PBR post processing.
	out.color = main_pass_post_lighting_processing(pbr_input, out.color);
#endif

  return out;
}
