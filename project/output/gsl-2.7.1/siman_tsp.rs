#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
extern "C" {
    fn acos(_: libc::c_double) -> libc::c_double;
    fn cos(_: libc::c_double) -> libc::c_double;
    fn sin(_: libc::c_double) -> libc::c_double;
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
    static mut stdout: *mut _IO_FILE;
    fn fflush(__stream: *mut FILE) -> i32;
    fn printf(_: *const i8, _: ...) -> i32;
    fn gsl_rng_env_setup() -> *const gsl_rng_type;
    fn gsl_rng_alloc(T: *const gsl_rng_type) -> *mut gsl_rng;
    fn gsl_siman_solve(
        r: *const gsl_rng,
        x0_p: *mut libc::c_void,
        Ef: gsl_siman_Efunc_t,
        take_step: gsl_siman_step_t,
        distance: gsl_siman_metric_t,
        print_position: gsl_siman_print_t,
        copyfunc: gsl_siman_copy_t,
        copy_constructor: gsl_siman_copy_construct_t,
        destructor: gsl_siman_destroy_t,
        element_size: size_t,
        params_0: gsl_siman_params_t,
    );
    fn gsl_ieee_env_setup();
}
pub type __off_t = i64;
pub type __off64_t = i64;
pub type size_t = u64;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: i32,
    pub _IO_read_ptr: *mut i8,
    pub _IO_read_end: *mut i8,
    pub _IO_read_base: *mut i8,
    pub _IO_write_base: *mut i8,
    pub _IO_write_ptr: *mut i8,
    pub _IO_write_end: *mut i8,
    pub _IO_buf_base: *mut i8,
    pub _IO_buf_end: *mut i8,
    pub _IO_save_base: *mut i8,
    pub _IO_backup_base: *mut i8,
    pub _IO_save_end: *mut i8,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: i32,
    pub _flags2: i32,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [i8; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    pub __pad1: *mut libc::c_void,
    pub __pad2: *mut libc::c_void,
    pub __pad3: *mut libc::c_void,
    pub __pad4: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: i32,
    pub _unused2: [i8; 20],
}
pub type _IO_lock_t = ();
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_marker {
    pub _next: *mut _IO_marker,
    pub _sbuf: *mut _IO_FILE,
    pub _pos: i32,
}
pub type FILE = _IO_FILE;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_rng_type {
    pub name: *const i8,
    pub max: u64,
    pub min: u64,
    pub size: size_t,
    pub set: Option<unsafe extern "C" fn(*mut libc::c_void, u64) -> ()>,
    pub get: Option<unsafe extern "C" fn(*mut libc::c_void) -> u64>,
    pub get_double: Option<unsafe extern "C" fn(*mut libc::c_void) -> libc::c_double>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_rng {
    pub type_0: *const gsl_rng_type,
    pub state: *mut libc::c_void,
}
pub type gsl_siman_Efunc_t = Option<
    unsafe extern "C" fn(*mut libc::c_void) -> libc::c_double,
>;
pub type gsl_siman_step_t = Option<
    unsafe extern "C" fn(*const gsl_rng, *mut libc::c_void, libc::c_double) -> (),
>;
pub type gsl_siman_metric_t = Option<
    unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> libc::c_double,
>;
pub type gsl_siman_print_t = Option<unsafe extern "C" fn(*mut libc::c_void) -> ()>;
pub type gsl_siman_copy_t = Option<
    unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> (),
>;
pub type gsl_siman_copy_construct_t = Option<
    unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void,
>;
pub type gsl_siman_destroy_t = Option<unsafe extern "C" fn(*mut libc::c_void) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_siman_params_t {
    pub n_tries: i32,
    pub iters_fixed_T: i32,
    pub step_size: libc::c_double,
    pub k: libc::c_double,
    pub t_initial: libc::c_double,
    pub mu_t: libc::c_double,
    pub t_min: libc::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct s_tsp_city {
    pub name: *const i8,
    pub lat: libc::c_double,
    pub longitude: libc::c_double,
}
pub type Stsp_city = s_tsp_city;
#[inline]
unsafe extern "C" fn gsl_rng_get(mut r: *const gsl_rng) -> u64 {
    return ((*(*r).type_0).get).expect("non-null function pointer")((*r).state);
}
#[no_mangle]
pub static mut params: gsl_siman_params_t = {
    let mut init = gsl_siman_params_t {
        n_tries: 200 as i32,
        iters_fixed_T: 2000 as i32,
        step_size: 1.0f64,
        k: 1.0f64,
        t_initial: 5000.0f64,
        mu_t: 1.002f64,
        t_min: 5.0e-1f64,
    };
    init
};
#[no_mangle]
pub static mut cities: [Stsp_city; 12] = [
    {
        let mut init = s_tsp_city {
            name: b"Santa Fe\0" as *const u8 as *const i8,
            lat: 35.68f64,
            longitude: 105.95f64,
        };
        init
    },
    {
        let mut init = s_tsp_city {
            name: b"Phoenix\0" as *const u8 as *const i8,
            lat: 33.54f64,
            longitude: 112.07f64,
        };
        init
    },
    {
        let mut init = s_tsp_city {
            name: b"Albuquerque\0" as *const u8 as *const i8,
            lat: 35.12f64,
            longitude: 106.62f64,
        };
        init
    },
    {
        let mut init = s_tsp_city {
            name: b"Clovis\0" as *const u8 as *const i8,
            lat: 34.41f64,
            longitude: 103.20f64,
        };
        init
    },
    {
        let mut init = s_tsp_city {
            name: b"Durango\0" as *const u8 as *const i8,
            lat: 37.29f64,
            longitude: 107.87f64,
        };
        init
    },
    {
        let mut init = s_tsp_city {
            name: b"Dallas\0" as *const u8 as *const i8,
            lat: 32.79f64,
            longitude: 96.77f64,
        };
        init
    },
    {
        let mut init = s_tsp_city {
            name: b"Tesuque\0" as *const u8 as *const i8,
            lat: 35.77f64,
            longitude: 105.92f64,
        };
        init
    },
    {
        let mut init = s_tsp_city {
            name: b"Grants\0" as *const u8 as *const i8,
            lat: 35.15f64,
            longitude: 107.84f64,
        };
        init
    },
    {
        let mut init = s_tsp_city {
            name: b"Los Alamos\0" as *const u8 as *const i8,
            lat: 35.89f64,
            longitude: 106.28f64,
        };
        init
    },
    {
        let mut init = s_tsp_city {
            name: b"Las Cruces\0" as *const u8 as *const i8,
            lat: 32.34f64,
            longitude: 106.76f64,
        };
        init
    },
    {
        let mut init = s_tsp_city {
            name: b"Cortez\0" as *const u8 as *const i8,
            lat: 37.35f64,
            longitude: 108.58f64,
        };
        init
    },
    {
        let mut init = s_tsp_city {
            name: b"Gallup\0" as *const u8 as *const i8,
            lat: 35.52f64,
            longitude: 108.74f64,
        };
        init
    },
];
#[no_mangle]
pub static mut distance_matrix: [[libc::c_double; 12]; 12] = [[0.; 12]; 12];
#[no_mangle]
pub unsafe extern "C" fn city_distance(
    mut c1: Stsp_city,
    mut c2: Stsp_city,
) -> libc::c_double {
    let earth_radius: libc::c_double = 6375.000f64;
    let mut sla1: libc::c_double = sin(
        c1.lat * 3.14159265358979323846f64 / 180 as i32 as libc::c_double,
    );
    let mut cla1: libc::c_double = cos(
        c1.lat * 3.14159265358979323846f64 / 180 as i32 as libc::c_double,
    );
    let mut slo1: libc::c_double = sin(
        c1.longitude * 3.14159265358979323846f64 / 180 as i32 as libc::c_double,
    );
    let mut clo1: libc::c_double = cos(
        c1.longitude * 3.14159265358979323846f64 / 180 as i32 as libc::c_double,
    );
    let mut sla2: libc::c_double = sin(
        c2.lat * 3.14159265358979323846f64 / 180 as i32 as libc::c_double,
    );
    let mut cla2: libc::c_double = cos(
        c2.lat * 3.14159265358979323846f64 / 180 as i32 as libc::c_double,
    );
    let mut slo2: libc::c_double = sin(
        c2.longitude * 3.14159265358979323846f64 / 180 as i32 as libc::c_double,
    );
    let mut clo2: libc::c_double = cos(
        c2.longitude * 3.14159265358979323846f64 / 180 as i32 as libc::c_double,
    );
    let mut x1: libc::c_double = cla1 * clo1;
    let mut x2: libc::c_double = cla2 * clo2;
    let mut y1: libc::c_double = cla1 * slo1;
    let mut y2: libc::c_double = cla2 * slo2;
    let mut z1: libc::c_double = sla1;
    let mut z2: libc::c_double = sla2;
    let mut dot_product: libc::c_double = x1 * x2 + y1 * y2 + z1 * z2;
    let mut angle: libc::c_double = acos(dot_product);
    return angle * earth_radius;
}
#[no_mangle]
pub unsafe extern "C" fn Etsp(mut xp: *mut libc::c_void) -> libc::c_double {
    let mut route: *mut i32 = xp as *mut i32;
    let mut E: libc::c_double = 0 as i32 as libc::c_double;
    let mut i: u32 = 0;
    i = 0 as i32 as u32;
    while (i as u64)
        < (::core::mem::size_of::<[Stsp_city; 12]>() as u64)
            .wrapping_div(::core::mem::size_of::<Stsp_city>() as u64)
    {
        E
            += distance_matrix[*route.offset(i as isize)
                as usize][*route
                .offset(
                    (i.wrapping_add(1 as i32 as u32) as u64)
                        .wrapping_rem(
                            (::core::mem::size_of::<[Stsp_city; 12]>() as u64)
                                .wrapping_div(::core::mem::size_of::<Stsp_city>() as u64),
                        ) as isize,
                ) as usize];
        i = i.wrapping_add(1);
        i;
    }
    return E;
}
#[no_mangle]
pub unsafe extern "C" fn Mtsp(
    mut xp: *mut libc::c_void,
    mut yp: *mut libc::c_void,
) -> libc::c_double {
    let mut route1: *mut i32 = xp as *mut i32;
    let mut route2: *mut i32 = yp as *mut i32;
    let mut distance: libc::c_double = 0 as i32 as libc::c_double;
    let mut i: u32 = 0;
    i = 0 as i32 as u32;
    while (i as u64)
        < (::core::mem::size_of::<[Stsp_city; 12]>() as u64)
            .wrapping_div(::core::mem::size_of::<Stsp_city>() as u64)
    {
        distance
            += (if *route1.offset(i as isize) == *route2.offset(i as isize) {
                0 as i32
            } else {
                1 as i32
            }) as libc::c_double;
        i = i.wrapping_add(1);
        i;
    }
    return distance;
}
#[no_mangle]
pub unsafe extern "C" fn Stsp(
    mut r: *const gsl_rng,
    mut xp: *mut libc::c_void,
    mut step_size: libc::c_double,
) {
    let mut x1: i32 = 0;
    let mut x2: i32 = 0;
    let mut dummy: i32 = 0;
    let mut route: *mut i32 = xp as *mut i32;
    step_size = 0 as i32 as libc::c_double;
    x1 = (gsl_rng_get(r))
        .wrapping_rem(
            (::core::mem::size_of::<[Stsp_city; 12]>() as u64)
                .wrapping_div(::core::mem::size_of::<Stsp_city>() as u64)
                .wrapping_sub(1 as i32 as u64),
        )
        .wrapping_add(1 as i32 as u64) as i32;
    loop {
        x2 = (gsl_rng_get(r))
            .wrapping_rem(
                (::core::mem::size_of::<[Stsp_city; 12]>() as u64)
                    .wrapping_div(::core::mem::size_of::<Stsp_city>() as u64)
                    .wrapping_sub(1 as i32 as u64),
            )
            .wrapping_add(1 as i32 as u64) as i32;
        if !(x2 == x1) {
            break;
        }
    }
    dummy = *route.offset(x1 as isize);
    *route.offset(x1 as isize) = *route.offset(x2 as isize);
    *route.offset(x2 as isize) = dummy;
}
#[no_mangle]
pub unsafe extern "C" fn Ptsp(mut xp: *mut libc::c_void) {
    let mut i: u32 = 0;
    let mut route: *mut i32 = xp as *mut i32;
    printf(b"  [\0" as *const u8 as *const i8);
    i = 0 as i32 as u32;
    while (i as u64)
        < (::core::mem::size_of::<[Stsp_city; 12]>() as u64)
            .wrapping_div(::core::mem::size_of::<Stsp_city>() as u64)
    {
        printf(b" %d \0" as *const u8 as *const i8, *route.offset(i as isize));
        i = i.wrapping_add(1);
        i;
    }
    printf(b"]  \0" as *const u8 as *const i8);
}
unsafe fn main_0() -> i32 {
    let mut x_initial: [i32; 12] = [0; 12];
    let mut i: u32 = 0;
    let mut r: *const gsl_rng = gsl_rng_alloc(gsl_rng_env_setup());
    gsl_ieee_env_setup();
    prepare_distance_matrix();
    printf(b"# initial order of cities:\n\0" as *const u8 as *const i8);
    i = 0 as i32 as u32;
    while (i as u64)
        < (::core::mem::size_of::<[Stsp_city; 12]>() as u64)
            .wrapping_div(::core::mem::size_of::<Stsp_city>() as u64)
    {
        printf(b"# \"%s\"\n\0" as *const u8 as *const i8, cities[i as usize].name);
        x_initial[i as usize] = i as i32;
        i = i.wrapping_add(1);
        i;
    }
    printf(b"# distance matrix is:\n\0" as *const u8 as *const i8);
    print_distance_matrix();
    printf(
        b"# initial coordinates of cities (longitude and latitude)\n\0" as *const u8
            as *const i8,
    );
    i = 0 as i32 as u32;
    while (i as u64)
        < (::core::mem::size_of::<[Stsp_city; 12]>() as u64)
            .wrapping_div(::core::mem::size_of::<Stsp_city>() as u64)
            .wrapping_add(1 as i32 as u64)
    {
        printf(
            b"###initial_city_coord: %g %g \"%s\"\n\0" as *const u8 as *const i8,
            -cities[x_initial[(i as u64)
                    .wrapping_rem(
                        (::core::mem::size_of::<[Stsp_city; 12]>() as u64)
                            .wrapping_div(::core::mem::size_of::<Stsp_city>() as u64),
                    ) as usize] as usize]
                .longitude,
            cities[x_initial[(i as u64)
                    .wrapping_rem(
                        (::core::mem::size_of::<[Stsp_city; 12]>() as u64)
                            .wrapping_div(::core::mem::size_of::<Stsp_city>() as u64),
                    ) as usize] as usize]
                .lat,
            cities[x_initial[(i as u64)
                    .wrapping_rem(
                        (::core::mem::size_of::<[Stsp_city; 12]>() as u64)
                            .wrapping_div(::core::mem::size_of::<Stsp_city>() as u64),
                    ) as usize] as usize]
                .name,
        );
        i = i.wrapping_add(1);
        i;
    }
    gsl_siman_solve(
        r,
        x_initial.as_mut_ptr() as *mut libc::c_void,
        Some(Etsp as unsafe extern "C" fn(*mut libc::c_void) -> libc::c_double),
        Some(
            Stsp
                as unsafe extern "C" fn(
                    *const gsl_rng,
                    *mut libc::c_void,
                    libc::c_double,
                ) -> (),
        ),
        Some(
            Mtsp
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *mut libc::c_void,
                ) -> libc::c_double,
        ),
        Some(Ptsp as unsafe extern "C" fn(*mut libc::c_void) -> ()),
        None,
        None,
        None,
        (::core::mem::size_of::<[Stsp_city; 12]>() as u64)
            .wrapping_div(::core::mem::size_of::<Stsp_city>() as u64)
            .wrapping_mul(::core::mem::size_of::<i32>() as u64),
        params,
    );
    printf(b"# final order of cities:\n\0" as *const u8 as *const i8);
    i = 0 as i32 as u32;
    while (i as u64)
        < (::core::mem::size_of::<[Stsp_city; 12]>() as u64)
            .wrapping_div(::core::mem::size_of::<Stsp_city>() as u64)
    {
        printf(
            b"# \"%s\"\n\0" as *const u8 as *const i8,
            cities[x_initial[i as usize] as usize].name,
        );
        i = i.wrapping_add(1);
        i;
    }
    printf(
        b"# final coordinates of cities (longitude and latitude)\n\0" as *const u8
            as *const i8,
    );
    i = 0 as i32 as u32;
    while (i as u64)
        < (::core::mem::size_of::<[Stsp_city; 12]>() as u64)
            .wrapping_div(::core::mem::size_of::<Stsp_city>() as u64)
            .wrapping_add(1 as i32 as u64)
    {
        printf(
            b"###final_city_coord: %g %g %s\n\0" as *const u8 as *const i8,
            -cities[x_initial[(i as u64)
                    .wrapping_rem(
                        (::core::mem::size_of::<[Stsp_city; 12]>() as u64)
                            .wrapping_div(::core::mem::size_of::<Stsp_city>() as u64),
                    ) as usize] as usize]
                .longitude,
            cities[x_initial[(i as u64)
                    .wrapping_rem(
                        (::core::mem::size_of::<[Stsp_city; 12]>() as u64)
                            .wrapping_div(::core::mem::size_of::<Stsp_city>() as u64),
                    ) as usize] as usize]
                .lat,
            cities[x_initial[(i as u64)
                    .wrapping_rem(
                        (::core::mem::size_of::<[Stsp_city; 12]>() as u64)
                            .wrapping_div(::core::mem::size_of::<Stsp_city>() as u64),
                    ) as usize] as usize]
                .name,
        );
        i = i.wrapping_add(1);
        i;
    }
    printf(b"# \0" as *const u8 as *const i8);
    fflush(stdout);
    fflush(stdout);
    return 0 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn prepare_distance_matrix() {
    let mut i: u32 = 0;
    let mut j: u32 = 0;
    let mut dist: libc::c_double = 0.;
    i = 0 as i32 as u32;
    while (i as u64)
        < (::core::mem::size_of::<[Stsp_city; 12]>() as u64)
            .wrapping_div(::core::mem::size_of::<Stsp_city>() as u64)
    {
        j = 0 as i32 as u32;
        while (j as u64)
            < (::core::mem::size_of::<[Stsp_city; 12]>() as u64)
                .wrapping_div(::core::mem::size_of::<Stsp_city>() as u64)
        {
            if i == j {
                dist = 0 as i32 as libc::c_double;
            } else {
                dist = city_distance(cities[i as usize], cities[j as usize]);
            }
            distance_matrix[i as usize][j as usize] = dist;
            j = j.wrapping_add(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
}
#[no_mangle]
pub unsafe extern "C" fn print_distance_matrix() {
    let mut i: u32 = 0;
    let mut j: u32 = 0;
    i = 0 as i32 as u32;
    while (i as u64)
        < (::core::mem::size_of::<[Stsp_city; 12]>() as u64)
            .wrapping_div(::core::mem::size_of::<Stsp_city>() as u64)
    {
        printf(b"# \0" as *const u8 as *const i8);
        j = 0 as i32 as u32;
        while (j as u64)
            < (::core::mem::size_of::<[Stsp_city; 12]>() as u64)
                .wrapping_div(::core::mem::size_of::<Stsp_city>() as u64)
        {
            printf(
                b"%15.8f   \0" as *const u8 as *const i8,
                distance_matrix[i as usize][j as usize],
            );
            j = j.wrapping_add(1);
            j;
        }
        printf(b"\n\0" as *const u8 as *const i8);
        i = i.wrapping_add(1);
        i;
    }
}
static mut best_E: libc::c_double = 1.0e100f64;
static mut second_E: libc::c_double = 1.0e100f64;
static mut third_E: libc::c_double = 1.0e100f64;
static mut best_route: [i32; 12] = [0; 12];
static mut second_route: [i32; 12] = [0; 12];
static mut third_route: [i32; 12] = [0; 12];
#[no_mangle]
pub unsafe extern "C" fn exhaustive_search() {
    static mut initial_route: [i32; 12] = [
        0 as i32,
        1 as i32,
        2 as i32,
        3 as i32,
        4 as i32,
        5 as i32,
        6 as i32,
        7 as i32,
        8 as i32,
        9 as i32,
        10 as i32,
        11 as i32,
    ];
    printf(b"\n# \0" as *const u8 as *const i8);
    fflush(stdout);
    fflush(stdout);
    do_all_perms(initial_route.as_mut_ptr(), 1 as i32);
    printf(b"\n# \0" as *const u8 as *const i8);
    fflush(stdout);
    fflush(stdout);
    printf(b"# exhaustive best route: \0" as *const u8 as *const i8);
    Ptsp(best_route.as_mut_ptr() as *mut libc::c_void);
    printf(b"\n# its energy is: %g\n\0" as *const u8 as *const i8, best_E);
    printf(b"# exhaustive second_best route: \0" as *const u8 as *const i8);
    Ptsp(second_route.as_mut_ptr() as *mut libc::c_void);
    printf(b"\n# its energy is: %g\n\0" as *const u8 as *const i8, second_E);
    printf(b"# exhaustive third_best route: \0" as *const u8 as *const i8);
    Ptsp(third_route.as_mut_ptr() as *mut libc::c_void);
    printf(b"\n# its energy is: %g\n\0" as *const u8 as *const i8, third_E);
}
unsafe extern "C" fn do_all_perms(mut route: *mut i32, mut n: i32) {
    if n as u64
        == (::core::mem::size_of::<[Stsp_city; 12]>() as u64)
            .wrapping_div(::core::mem::size_of::<Stsp_city>() as u64)
            .wrapping_sub(1 as i32 as u64)
    {
        let mut E: libc::c_double = 0.;
        E = Etsp(route as *mut libc::c_void);
        if E < best_E {
            third_E = second_E;
            memcpy(
                third_route.as_mut_ptr() as *mut libc::c_void,
                second_route.as_mut_ptr() as *const libc::c_void,
                (::core::mem::size_of::<[Stsp_city; 12]>() as u64)
                    .wrapping_div(::core::mem::size_of::<Stsp_city>() as u64)
                    .wrapping_mul(::core::mem::size_of::<i32>() as u64),
            );
            second_E = best_E;
            memcpy(
                second_route.as_mut_ptr() as *mut libc::c_void,
                best_route.as_mut_ptr() as *const libc::c_void,
                (::core::mem::size_of::<[Stsp_city; 12]>() as u64)
                    .wrapping_div(::core::mem::size_of::<Stsp_city>() as u64)
                    .wrapping_mul(::core::mem::size_of::<i32>() as u64),
            );
            best_E = E;
            memcpy(
                best_route.as_mut_ptr() as *mut libc::c_void,
                route as *const libc::c_void,
                (::core::mem::size_of::<[Stsp_city; 12]>() as u64)
                    .wrapping_div(::core::mem::size_of::<Stsp_city>() as u64)
                    .wrapping_mul(::core::mem::size_of::<i32>() as u64),
            );
        } else if E < second_E {
            third_E = second_E;
            memcpy(
                third_route.as_mut_ptr() as *mut libc::c_void,
                second_route.as_mut_ptr() as *const libc::c_void,
                (::core::mem::size_of::<[Stsp_city; 12]>() as u64)
                    .wrapping_div(::core::mem::size_of::<Stsp_city>() as u64)
                    .wrapping_mul(::core::mem::size_of::<i32>() as u64),
            );
            second_E = E;
            memcpy(
                second_route.as_mut_ptr() as *mut libc::c_void,
                route as *const libc::c_void,
                (::core::mem::size_of::<[Stsp_city; 12]>() as u64)
                    .wrapping_div(::core::mem::size_of::<Stsp_city>() as u64)
                    .wrapping_mul(::core::mem::size_of::<i32>() as u64),
            );
        } else if E < third_E {
            third_E = E;
            memcpy(
                route as *mut libc::c_void,
                third_route.as_mut_ptr() as *const libc::c_void,
                (::core::mem::size_of::<[Stsp_city; 12]>() as u64)
                    .wrapping_div(::core::mem::size_of::<Stsp_city>() as u64)
                    .wrapping_mul(::core::mem::size_of::<i32>() as u64),
            );
        }
    } else {
        let mut new_route: [i32; 12] = [0; 12];
        let mut j: u32 = 0;
        let mut swap_tmp: i32 = 0;
        memcpy(
            new_route.as_mut_ptr() as *mut libc::c_void,
            route as *const libc::c_void,
            (::core::mem::size_of::<[Stsp_city; 12]>() as u64)
                .wrapping_div(::core::mem::size_of::<Stsp_city>() as u64)
                .wrapping_mul(::core::mem::size_of::<i32>() as u64),
        );
        j = n as u32;
        while (j as u64)
            < (::core::mem::size_of::<[Stsp_city; 12]>() as u64)
                .wrapping_div(::core::mem::size_of::<Stsp_city>() as u64)
        {
            swap_tmp = new_route[j as usize];
            new_route[j as usize] = new_route[n as usize];
            new_route[n as usize] = swap_tmp;
            do_all_perms(new_route.as_mut_ptr(), n + 1 as i32);
            j = j.wrapping_add(1);
            j;
        }
    };
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}