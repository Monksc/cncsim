
module rectangle(l, w, r) {
    translate([0, 0, 0.5])
    union() {
        translate([r, 0, 0])
            square([l-2*r, w]);
        translate([0, r, 0])
            square([l, w-2*r]);
        translate([r, r, 0])
            circle(r);
        translate([l-r, r, 0])
            circle(r);
        translate([r, w-r, 0])
            circle(r);
        translate([l-r, w-r, 0])
            circle(r);
    };
}

module bitpathbasic(l, h, r) {
    translate([0, -r, r])
    union() {
        translate([0,0,0])
            cube([l,2*r, h]);
        translate([0, r, 0])
            rotate([0,90,0])
            cylinder(l, r, r);
    }
}

function quadratic(x, y, z, j, k, l) =
    sqrt(pow(x-j, 2) + pow(y-k, 2) + pow(z-l,2));

module bitpath(x, y, z, j, k, l, h, r) {

    // distance = quadratic(x, y, z, j, k, l);
    distance = quadratic(x, y, 0, j, k, 0);
    xangle = atan((z-l) / (y-k));
    yangle = atan((x-j) / (z-l));
    zangle = x==j ? (y>k ? 90 : -90) : (y==k ? (x > j ? 180 : 0) : atan((y-k) / (x-j)));

    // translate([min(x, j), min(y, k), min(z, l)])
    translate([x, y, z])
        rotate([0, 0, -zangle])
        bitpathbasic(distance, h, r);
}

difference() {
    translate([0, 0, -0.5])
