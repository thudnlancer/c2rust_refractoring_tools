//! Integration using 61-point Gauss-Kronrod rule

/// Gauss quadrature weights and kronrod quadrature abscissae and
/// weights as evaluated with 80 decimal digit arithmetic by
/// L. W. Fullerton, Bell Labs, Nov. 1981.

const XGK: [f64; 31] = [
    // abscissae of the 61-point kronrod rule
    0.999484410050490637571325895705811,
    0.996893484074649540271630050918695,
    0.991630996870404594858628366109486,
    0.983668123279747209970032581605663,
    0.973116322501126268374693868423707,
    0.960021864968307512216871025581798,
    0.944374444748559979415831324037439,
    0.926200047429274325879324277080474,
    0.905573307699907798546522558925958,
    0.882560535792052681543116462530226,
    0.857205233546061098958658510658944,
    0.829565762382768397442898119732502,
    0.799727835821839083013668942322683,
    0.767777432104826194917977340974503,
    0.733790062453226804726171131369528,
    0.697850494793315796932292388026640,
    0.660061064126626961370053668149271,
    0.620526182989242861140477556431189,
    0.579345235826361691756024932172540,
    0.536624148142019899264169793311073,
    0.492480467861778574993693061207709,
    0.447033769538089176780609900322854,
    0.400401254830394392535476211542661,
    0.352704725530878113471037207089374,
    0.304073202273625077372677107199257,
    0.254636926167889846439805129817805,
    0.204525116682309891438957671002025,
    0.153869913608583546963794672743256,
    0.102806937966737030147096751318001,
    0.051471842555317695833025213166723,
    0.000000000000000000000000000000000,
];

/// xgk[1], xgk[3], ... abscissae of the 30-point gauss rule.
/// xgk[0], xgk[2], ... abscissae to optimally extend the 30-point gauss rule

const WG: [f64; 15] = [
    // weights of the 30-point gauss rule
    0.007968192496166605615465883474674,
    0.018466468311090959142302131912047,
    0.028784707883323369349719179611292,
    0.038799192569627049596801936446348,
    0.048402672830594052902938140422808,
    0.057493156217619066481721689402056,
    0.065974229882180495128128515115962,
    0.073755974737705206268243850022191,
    0.080755895229420215354694938460530,
    0.086899787201082979802387530715126,
    0.092122522237786128717632707087619,
    0.096368737174644259639468626351810,
    0.099593420586795267062780282103569,
    0.101762389748405504596428952168554,
    0.102852652893558840341285636705415,
];

const WGK: [f64; 31] = [
    // weights of the 61-point kronrod rule
    0.001389013698677007624551591226760,
    0.003890461127099884051267201844516,
    0.006630703915931292173319826369750,
    0.009273279659517763428441146892024,
    0.011823015253496341742232898853251,
    0.014369729507045804812451432443580,
    0.016920889189053272627572289420322,
    0.019414141193942381173408951050128,
    0.021828035821609192297167485738339,
    0.024191162078080601365686370725232,
    0.026509954882333101610601709335075,
    0.028754048765041292843978785354334,
    0.030907257562387762472884252943092,
    0.032981447057483726031814191016854,
    0.034979338028060024137499670731468,
    0.036882364651821229223911065617136,
    0.038678945624727592950348651532281,
    0.040374538951535959111995279752468,
    0.041969810215164246147147541285970,
    0.043452539701356069316831728117073,
    0.044814800133162663192355551616723,
    0.046059238271006988116271735559374,
    0.047185546569299153945261478181099,
    0.048185861757087129140779492298305,
    0.049055434555029778887528165367238,
    0.049795683427074206357811569379942,
    0.050405921402782346840893085653585,
    0.050881795898749606492297473049805,
    0.051221547849258772170656282604944,
    0.051426128537459025933862879215781,
    0.051494729429451567558340433647099,
];

/// Performs 61-point Gauss-Kronrod integration
pub fn gsl_integration_qk61<F>(f: F, a: f64, b: f64) -> (f64, f64, f64, f64)
where
    F: Fn(f64) -> f64,
{
    let mut fv1 = [0.0; 31];
    let mut fv2 = [0.0; 31];
    
    gsl_integration_qk(
        31,
        &XGK,
        &WG,
        &WGK,
        &mut fv1,
        &mut fv2,
        f,
        a,
        b,
    )
}

fn gsl_integration_qk<F>(
    n: usize,
    xgk: &[f64],
    wg: &[f64],
    wgk: &[f64],
    fv1: &mut [f64],
    fv2: &mut [f64],
    f: F,
    a: f64,
    b: f64,
) -> (f64, f64, f64, f64)
where
    F: Fn(f64) -> f64,
{
    let center = 0.5 * (a + b);
    let half_length = 0.5 * (b - a);
    let abs_half_length = half_length.abs();
    let f_center = f(center);

    let mut result_gauss = 0.0;
    let mut result_kronrod = f_center * wgk[n - 1];
    let mut result_abs = result_kronrod.abs();
    let mut result_asc: f64;
    let mut mean: f64;
    let mut err: f64;

    if n % 2 == 0 {
        result_gauss = f_center * wg[n / 2 - 1];
    }

    for j in 0..(n / 2) {
        let jtw = j * 2 + 1;
        let abscissa = half_length * xgk[jtw];
        let fval1 = f(center - abscissa);
        let fval2 = f(center + abscissa);
        let fsum = fval1 + fval2;

        fv1[jtw] = fval1;
        fv2[jtw] = fval2;
        result_kronrod += wgk[jtw] * fsum;
        result_abs += wgk[jtw] * (fval1.abs() + fval2.abs());

        if j % 2 == 1 {
            let idx = (j + 1) / 2 - 1;
            result_gauss += wg[idx] * fsum;
        }
    }

    for j in 0..(n / 2) {
        let jtwm1 = j * 2;
        let abscissa = half_length * xgk[jtwm1];
        let fval1 = f(center - abscissa);
        let fval2 = f(center + abscissa);
        fv1[jtwm1] = fval1;
        fv2[jtwm1] = fval2;
        result_kronrod += wgk[jtwm1] * (fval1 + fval2);
        result_abs += wgk[jtwm1] * (fval1.abs() + fval2.abs());
    }

    mean = result_kronrod * 0.5;
    result_asc = wgk[n - 1] * (f_center - mean).abs();

    for j in 0..(n - 1) {
        result_asc += wgk[j] * (fv1[j].abs() + fv2[j].abs() - 2.0 * mean.abs());
    }

    // Scale results
    let scale_err = result_asc * abs_half_length;
    result_kronrod *= half_length;
    result_abs *= abs_half_length;
    result_gauss *= half_length;
    result_asc = scale_err;

    if result_asc != 0.0 && err != 0.0 {
        let error_ratio = (200.0 * err / result_asc).abs();
        if error_ratio > 1.0 {
            err = result_asc * error_ratio.min(1.0);
        }
    }

    if result_abs > f64::MIN_POSITIVE / (50.0 * f64::EPSILON) {
        let min_err = 50.0 * f64::EPSILON * result_abs;
        if min_err > err {
            err = min_err;
        }
    }

    (
        result_kronrod,
        err,
        result_abs,
        result_asc,
    )
}