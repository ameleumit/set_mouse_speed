
fn main() {
	 println!("cargo:rerun-if-changed={}",env!("CARGO_MANIFEST_DIR"));

	let (desc,pkg_name,homepage,repository,licence,_licence_file,authors)=(
		env!("CARGO_PKG_DESCRIPTION"),
		env!("CARGO_PKG_NAME"),	//set_mouse_speed
		env!("CARGO_PKG_HOMEPAGE"),		//boş
		env!("CARGO_PKG_REPOSITORY"),	//boş
		env!("CARGO_PKG_LICENSE"),
		env!("CARGO_PKG_LICENSE_FILE"),
		env!("CARGO_PKG_AUTHORS"),

	);

	 let (major, minor, patch) = (
        env!("CARGO_PKG_VERSION_MAJOR"),
        env!("CARGO_PKG_VERSION_MINOR"),
        env!("CARGO_PKG_VERSION_PATCH"),
    );
    let version = env!("CARGO_PKG_VERSION");

	let icon = "res/exe_icon.ico";

	

	let rc = format!(
        r#"	1 ICON "{icon}"

			1 VERSIONINFO
			FILEVERSION {major},{minor},{patch},0
			PRODUCTVERSION {major},{minor},{patch},0
			FILEOS 0x40004L
			FILETYPE 0x1L
			BEGIN
				BLOCK "StringFileInfo"
				BEGIN
					BLOCK "000004E6"
					BEGIN
						VALUE "Comments", "{homepage}"
						VALUE "CompanyName", "Umit UZUN-Ümit UZUN"
						VALUE "FileVersion", "{version}"
						VALUE "FileDescription", "{desc}"
						VALUE "InternalName", "{authors}"
						VALUE "LegalTrademarks", "main_main.rs"
						VALUE "LegalCopyright", "{licence}"
						VALUE "OriginalFilename", "{pkg_name}"
						VALUE "PrivateBuild", "{authors}"
						// VALUE "ProductName", "{repository}"
						VALUE "ProductName", "{authors}"
						VALUE "ProductVersion", "{version}"
						VALUE "SpecialBuild", "{authors}"
					END
				END
				BLOCK "VarFileInfo"
				BEGIN
					VALUE "Translation", 0x0000, 1254
				END
			END
			"#
    );

	
    let rc_path = "res/resource.rc";
    std::fs::write(&rc_path, rc).expect("write generated .rc into OUT_DIR");
    embed_resource::compile(&rc_path, embed_resource::NONE)
        .manifest_optional()
        .expect("compile Windows resources");
}


