use windows::{core::*, Win32::Graphics::Gdi::*, Win32::Graphics::OpenGL::*};

pub type GLboolean = u32;
// pub type GLbyte = i8;
// pub type GLubyte = u8;
pub type GLchar = std::os::raw::c_char;
// pub type GLshort = i16;
// pub type GLushort = u16;
pub type GLint = i32;
pub type GLuint = u32;
// pub type GLfixed = i32;
// pub type GLint64 = i64;
// pub type GLuint64 = u64;
pub type GLsizei = i32;
pub type GLenum = u32;
// pub type GLintptr = *const i32;
pub type GLsizeiptr = i32;
// pub type GLsync = *const i32;
// pub type GLbitfield = i32;
// pub type GLhalf = f16;
pub type GLfloat = f32;
// pub type GLclampf = f32;
// pub type GLdouble = f64;
// pub type GLclampd = f64;

#[allow(non_snake_case)]
pub struct Extend {
    pub glGenBuffers: extern "C" fn(GLsizei, *const GLuint),
    pub glBindBuffer: extern "C" fn(GLenum, GLuint),
    pub glBufferData: extern "C" fn(GLenum, GLsizeiptr, *const std::ffi::c_void, GLenum),
    pub glCreateShader: extern "C" fn(GLenum) -> GLuint,
    pub glShaderSource: extern "C" fn(GLuint, GLsizei, *const *const GLchar, *const GLint),
    pub glCompileShader: extern "C" fn(GLuint),
    pub glCreateProgram: extern "C" fn() -> GLuint,
    pub glAttachShader: extern "C" fn(GLuint, GLuint),
    pub glLinkProgram: extern "C" fn(GLuint),
    pub glUseProgram: extern "C" fn(GLuint),
    pub glGetAttribLocation: extern "C" fn(GLuint, *const GLchar) -> GLint,
    pub glEnableVertexAttribArray: extern "C" fn(GLuint),
    pub glVertexAttribPointer: extern "C" fn(GLuint, GLint, GLenum, GLboolean, GLsizei, *const std::ffi::c_void),
    pub wglCreateContextAttribsARB: extern "C" fn(HDC, HGLRC, *const i32) -> HGLRC,

    //shader
    vbo: [GLuint; 2],
    posAttrib: GLint,
    colAttrib: GLint,
}

impl Extend {
    pub fn new() -> Self {
        Self {
            glGenBuffers: Self::get_symbol(s!("glGenBuffers")).unwrap(),
            glBindBuffer: Self::get_symbol(s!("glBindBuffer")).unwrap(),
            glBufferData: Self::get_symbol(s!("glBufferData")).unwrap(),
            glCreateShader: Self::get_symbol(s!("glCreateShader")).unwrap(),
            glShaderSource: Self::get_symbol(s!("glShaderSource")).unwrap(),
            glCompileShader: Self::get_symbol(s!("glCompileShader")).unwrap(),
            glCreateProgram: Self::get_symbol(s!("glCreateProgram")).unwrap(),
            glAttachShader: Self::get_symbol(s!("glAttachShader")).unwrap(),
            glLinkProgram: Self::get_symbol(s!("glLinkProgram")).unwrap(),
            glUseProgram: Self::get_symbol(s!("glUseProgram")).unwrap(),
            glGetAttribLocation: Self::get_symbol(s!("glGetAttribLocation")).unwrap(),
            glEnableVertexAttribArray: Self::get_symbol(s!("glEnableVertexAttribArray")).unwrap(),
            glVertexAttribPointer: Self::get_symbol(s!("glVertexAttribPointer")).unwrap(),
            wglCreateContextAttribsARB: Self::get_symbol(s!("wglCreateContextAttribsARB")).unwrap(),

            vbo: [0, 0],
            posAttrib: 0,
            colAttrib: 0,
        }
    }

    fn get_symbol<F: Sized>(name: PCSTR) -> std::result::Result<F, i32> {
        let func_addr = unsafe { wglGetProcAddress(name) }.ok_or(0)?;
        Ok(unsafe { std::mem::transmute_copy(&func_addr) })
    }

    pub fn new_shader(&mut self) {
        let slice: &[GLuint] = &self.vbo;
        let rawptr: *const GLuint = &slice[0] as *const GLuint;
        (self.glGenBuffers)(2, rawptr);

        type BufferType = [GLfloat; 9];
        let buffer_size = std::mem::size_of::<BufferType>() as i32;

        let vertices: BufferType = [0.0, 0.5, 0.0, 0.5, -0.5, 0.0, -0.5, -0.5, 0.0];
        let vslice: &[GLfloat] = &vertices;
        let vrawptr = &vslice[0] as *const GLfloat as *const std::ffi::c_void;

        let colors: BufferType = [1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0];
        let cslice: &[GLfloat] = &colors;
        let crawptr = &cslice[0] as *const GLfloat as *const std::ffi::c_void;

        const GL_ARRAY_BUFFER: GLenum = 0x8892;
        const GL_STATIC_DRAW: GLenum = 0x88E4;
        (self.glBindBuffer)(GL_ARRAY_BUFFER, self.vbo[0]);
        (self.glBufferData)(GL_ARRAY_BUFFER, buffer_size, vrawptr, GL_STATIC_DRAW);

        (self.glBindBuffer)(GL_ARRAY_BUFFER, self.vbo[1]);
        (self.glBufferData)(GL_ARRAY_BUFFER, buffer_size, crawptr, GL_STATIC_DRAW);

        const GL_FRAGMENT_SHADER: GLenum = 0x8B30;
        const GL_VERTEX_SHADER: GLenum = 0x8B31;
        const FRAGMENT_SOURCE: &str = include_str!("./fragment_shader.glsl");
        const VERTEX_SOURCE: &str = include_str!("./vertex_shader.glsl");
        let f_cstring = std::ffi::CString::new(FRAGMENT_SOURCE).unwrap();
        let v_cstring = std::ffi::CString::new(VERTEX_SOURCE).unwrap();
        let f_cstring: *const *const GLchar = &f_cstring.as_ptr() as _;
        let v_cstring: *const *const GLchar = &v_cstring.as_ptr() as _;

        // vertex shader
        let vertex_shader: GLuint = (self.glCreateShader)(GL_VERTEX_SHADER);
        (self.glShaderSource)(vertex_shader, 1, v_cstring, std::ptr::null());
        (self.glCompileShader)(vertex_shader);

        // fragment shader
        let fragment_shader: GLuint = (self.glCreateShader)(GL_FRAGMENT_SHADER);
        (self.glShaderSource)(fragment_shader, 1, f_cstring, std::ptr::null());
        (self.glCompileShader)(fragment_shader);

        let shader_program: GLuint = (self.glCreateProgram)();
        (self.glAttachShader)(shader_program, vertex_shader);
        (self.glAttachShader)(shader_program, fragment_shader);
        (self.glLinkProgram)(shader_program);
        (self.glUseProgram)(shader_program);

        let pos_string = std::ffi::CString::new("position").unwrap();
        self.posAttrib = (self.glGetAttribLocation)(shader_program, pos_string.as_ptr());
        assert_ne!(self.posAttrib, -1);
        (self.glEnableVertexAttribArray)(self.posAttrib as GLuint);

        let color_string = std::ffi::CString::new("color").unwrap();
        self.colAttrib = (self.glGetAttribLocation)(shader_program, color_string.as_ptr());
        assert_ne!(self.colAttrib, -1);
        (self.glEnableVertexAttribArray)(self.colAttrib as GLuint);
    }

    pub fn draw_triangle(&self) {
        const GL_ARRAY_BUFFER: GLenum = 0x8892;

        (self.glBindBuffer)(GL_ARRAY_BUFFER, self.vbo[0]);
        (self.glVertexAttribPointer)(self.posAttrib as GLuint, 3, GL_FLOAT, GL_FALSE, 0, std::ptr::null());

        (self.glBindBuffer)(GL_ARRAY_BUFFER, self.vbo[1]);
        (self.glVertexAttribPointer)(self.colAttrib as GLuint, 3, GL_FLOAT, GL_FALSE, 0, std::ptr::null());

        unsafe {
            glClearColor(0.0, 0.0, 0.0, 0.0);
            glClear(GL_COLOR_BUFFER_BIT);
            glDrawArrays(GL_TRIANGLES, 0, 3);
        }
    }
}
