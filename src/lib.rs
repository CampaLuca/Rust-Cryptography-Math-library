#![recursion_limit = "2048"]
pub mod poly {
    pub mod classes {
        pub mod polynomial;
        pub mod monomial;
        pub mod univariate_polynomial;
    }

    pub mod instances {
    pub mod polynomial_instance;
    pub mod monomial_instance;
    pub mod univariate_polynomial_instance;
    }
}
pub mod numbers {
    pub mod sets {
        pub mod Class;
        
    }
    pub mod numbers;
    pub mod classes {
        pub mod QQ;
        pub mod RR;
        pub mod ZZ;
    }

    pub mod instances {
        pub mod QQ_instance;
        pub mod RR_instance;
        pub mod ZZ_instance;
    }
    
}

pub mod matrices {
    pub mod matrix;
    pub mod vector;
}

pub mod variables {
    pub mod vars;
}

pub mod utilities {
    pub mod utils;
}

pub mod test {
    pub mod test_ZZ;
    pub mod test_RR;
    pub mod test_QQ;
}

pub mod algebras {

    pub mod Rings {
        pub mod classes {
            pub mod PolynomialRing;
        }

        pub mod instances {
            pub mod PolynomialRing_instance;
        }

    }

    pub mod Groups {
        pub mod EllipticCurves {
            pub mod elliptic_curve_factory;
            pub mod elliptic_curve_finite_field;
            pub mod elliptic_curve_number_field;
            pub mod elliptic_curve_generic;
            pub mod elliptic_curve_rational_field;
            pub mod elliptic_curve_point;
            pub mod elliptic_curve_polynomial_ring;
        }
    }


    pub mod FiniteField {
        pub mod classes {
            pub mod Zmod;
        }

        pub mod instances {
            pub mod Zmod_instance;
        }
    }
}

pub mod transform {
    pub mod ntt;
}

pub mod arith {
    pub mod random;
    pub mod primes;
}

pub mod cryptography {
    pub mod asymmetric {
        pub mod interfaces { pub mod interfaces; }
        pub mod primitives { pub mod rsa; pub mod kyber;}
    }


    pub mod homomorphic {
        pub mod interfaces {
            pub mod interfaces;
        }

        pub mod primitives {
            pub mod bfv;
        }
    }

    pub mod symmetric {
        pub mod AES {
            pub mod aes_functions;
            pub mod cipher;
            pub mod data;
        }
        pub mod modes {
            pub mod modes;
        }

        pub mod interfaces { pub mod interfaces; }
        pub mod primitives { pub mod aes; }
    }

    pub mod padding {
        pub mod padding;
    }
}

