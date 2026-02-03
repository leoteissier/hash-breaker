use hash_breaker::hashing::{detect_algorithm, hash_password, verify_password, SaltPosition};

#[test]
fn test_md5_hash() {
    let password = "password";
    let hash = hash_password(password, "md5", "", SaltPosition::After);
    assert_eq!(hash, "5f4dcc3b5aa765d61d8327deb882cf99");
}

#[test]
fn test_sha1_hash() {
    let password = "password";
    let hash = hash_password(password, "sha1", "", SaltPosition::After);
    assert_eq!(hash, "5baa61e4c9b93f3f0682250b6cf8331b7ee68fd8");
}

#[test]
fn test_sha256_hash() {
    let password = "password";
    let hash = hash_password(password, "sha256", "", SaltPosition::After);
    assert_eq!(
        hash,
        "5e884898da28047151d0e56f8dc6292773603d0d6aabbdd62a11ef721d1542d8"
    );
}

#[test]
fn test_sha512_hash() {
    let password = "password";
    let hash = hash_password(password, "sha512", "", SaltPosition::After);
    assert_eq!(
        hash,
        "b109f3bbbc244eb82441917ed06d618b9008dd09b3befd1b5e07394c706a8bb980b1d7785e5976ec049b46df5f1326af5a2ea6d103fd07c95385ffab0cacbc86"
    );
}

#[test]
fn test_bcrypt_hash() {
    let password = "password";
    let hash = hash_password(password, "bcrypt", "", SaltPosition::After);
    assert!(hash.starts_with("$2b$"));
}

#[test]
fn test_argon2_hash() {
    let password = "password";
    let hash = hash_password(password, "argon2", "", SaltPosition::After);
    assert!(hash.starts_with("$argon2"));
}

#[test]
fn test_base64_hash() {
    let password = "password";
    let hash = hash_password(password, "base64", "", SaltPosition::After);
    assert_eq!(hash, "cGFzc3dvcmQ=");
}

#[test]
fn test_invalid_algorithm() {
    let password = "password";
    let result = std::panic::catch_unwind(|| {
        hash_password(password, "invalid_algo", "", SaltPosition::After);
    });
    assert!(result.is_err());
}

/// Tests pour la détection d'algorithme

#[test]
fn test_detect_md5_algorithm() {
    let hash = "5f4dcc3b5aa765d61d8327deb882cf99";
    let detected_algo = detect_algorithm(hash).unwrap();
    assert_eq!(detected_algo, "md5");
}

#[test]
fn test_detect_sha1_algorithm() {
    let hash = "5baa61e4c9b93f3f0682250b6cf8331b7ee68fd8";
    let detected_algo = detect_algorithm(hash).unwrap();
    assert_eq!(detected_algo, "sha1");
}

#[test]
fn test_detect_sha256_algorithm() {
    let hash = "5e884898da28047151d0e56f8dc6292773603d0d6aabbdd62a11ef721d1542d8";
    let detected_algo = detect_algorithm(hash).unwrap();
    assert_eq!(detected_algo, "sha256");
}

#[test]
fn test_detect_sha512_algorithm() {
    let hash = "b109f3bbbc244eb82441917ed06d618b9008dd09b3befd1b5e07394c706a8bb980b1d7785e5976ec049b46df5f1326af5a2ea6d103fd07c95385ffab0cacbc86";
    let detected_algo = detect_algorithm(hash).unwrap();
    assert_eq!(detected_algo, "sha512");
}

#[test]
fn test_detect_bcrypt_algorithm() {
    let hash = "$2b$12$abcdefghijklmnopqrstuvxy";
    let detected_algo = detect_algorithm(hash).unwrap();
    assert_eq!(detected_algo, "bcrypt");
}

#[test]
fn test_detect_argon2_algorithm() {
    let hash = "$argon2i$v=19$m=4096,t=3,p=1$...";
    let detected_algo = detect_algorithm(hash).unwrap();
    assert_eq!(detected_algo, "argon2");
}

#[test]
fn test_detect_base64_algorithm() {
    let hash = "cGFzc3dvcmQ=";
    let detected_algo = detect_algorithm(hash).unwrap();
    assert_eq!(detected_algo, "base64");
}

#[test]
fn test_detect_invalid_algorithm() {
    let hash = "invalidhash";
    let result = detect_algorithm(hash);
    assert!(result.is_err());
}

/// Tests pour verify_password (bcrypt et argon2 utilisent le salt intégré au hash)
#[test]
fn test_verify_bcrypt() {
    // Hash bcrypt connu pour "password" (cost 4)
    let hash = hash_password("password", "bcrypt", "", SaltPosition::After);
    assert!(verify_password("password", &hash, "bcrypt", "", SaltPosition::After));
    assert!(!verify_password("wrong", &hash, "bcrypt", "", SaltPosition::After));
}

#[test]
fn test_verify_argon2() {
    let hash = hash_password("password", "argon2", "", SaltPosition::After);
    assert!(verify_password("password", &hash, "argon2", "", SaltPosition::After));
    assert!(!verify_password("wrong", &hash, "argon2", "", SaltPosition::After));
}

#[test]
fn test_verify_md5_with_salt() {
    let hash = hash_password("pass", "md5", "salt", SaltPosition::After);
    assert!(verify_password("pass", &hash, "md5", "salt", SaltPosition::After));
    assert!(!verify_password("pass", &hash, "md5", "salt", SaltPosition::Before));
}
