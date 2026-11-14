struct IceVertex { @builtin(position) clip: vec4f, @location(0) world: vec3f }
@vertex fn ice_vertex(@location(0) p: vec3f) -> IceVertex {
    var out: IceVertex;
    out.clip = project(p);
    out.world = p;
    return out;
}
@fragment fn ice_fragment(v: IceVertex) -> @location(0) vec4f {
    let n = normalize(cross(dpdx(v.world), dpdy(v.world)));
    let view = normalize(camera.eye.xyz - v.world);
    let sun = normalize(vec3f(0.4, 1.0, 0.3));
    let rim = pow(1.0 - abs(dot(n, view)), 3.0);
    let diffuse = 0.45 + 0.55 * abs(dot(n, sun));
    let sparkle = pow(abs(dot(n, normalize(sun + view))), 72.0);
    let ice = mix(vec3f(0.12, 0.48, 0.65), vec3f(0.65, 0.91, 1.0), rim);
    let color = ice * diffuse + vec3f(sparkle * 0.65);
    let fog = max(1.0 - exp(-0.025 * distance(camera.eye.xyz, v.world)), smoothstep(40.0, 60.0, distance(camera.eye.xz, v.world.xz)));
    return vec4f(mix(color, vec3f(0.47, 0.65, 0.78), fog), 0.82 + 0.16 * rim);
}

@fragment fn slush_fragment(v: IceVertex) -> @location(0) vec4f {
    let n = normalize(cross(dpdx(v.world), dpdy(v.world)));
    let light = 0.48 + 0.52 * abs(dot(n, normalize(vec3f(0.4, 1.0, 0.3))));
    let frost = 0.5 + 0.5 * sin(v.world.y * 14.0 + v.world.x * 2.0 + v.world.z * 3.0);
    let tint = mix(vec3f(0.25, 0.58, 0.72), vec3f(0.9, 0.97, 1.0), 0.48 + 0.17 * frost);
    let fog = max(1.0 - exp(-0.025 * distance(camera.eye.xyz, v.world)), smoothstep(40.0, 60.0, distance(camera.eye.xz, v.world.xz)));
    return vec4f(mix(tint * light, vec3f(0.47, 0.65, 0.78), fog), 0.98);
}

@fragment fn water_fragment(v: IceVertex) -> @location(0) vec4f {
    let n = normalize(cross(dpdx(v.world), dpdy(v.world)));
    let view = normalize(camera.eye.xyz - v.world);
    let rim = pow(1.0 - abs(dot(n, view)), 3.0);
    let light = normalize(vec3f(0.4, 1.0, 0.3));
    let sheen = pow(abs(dot(n, normalize(view + light))), 44.0);
    let foam = pow(0.5 + 0.5 * sin(v.world.y * 15.0 + v.world.x * 3.0), 12.0);
    let water = mix(vec3f(0.08, 0.43, 0.62), vec3f(0.75, 0.93, 1.0), rim);
    let color = water + vec3f(0.4 * sheen + 0.12 * foam);
    let fog = max(1.0 - exp(-0.025 * distance(camera.eye.xyz, v.world)), smoothstep(40.0, 60.0, distance(camera.eye.xz, v.world.xz)));
    return vec4f(mix(color, vec3f(0.47, 0.65, 0.78), fog), 0.65 + 0.3 * rim);
}
