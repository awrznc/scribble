// @ts-check

class Point {
  /** @type {Number} */
  x;
  /** @type {Number} */
  y;
  constructor() {
    this.x = 0;
    this.y = 0;
  }
}

export class App {
  /** @type {Number} */
  width;
  /** @type {Number} */
  height;

  /**
   * @param {HTMLElement} target
   * @param {string} image
   * @param {any} THREE
   * @param {any} OrbitControls
   */
  constructor(target, image, THREE, OrbitControls) {
    const self = this;
    self.three = THREE;
    self.mouse = new Point();
    self.before = new Point();
    self.after = new Point();

    self.updateWindowSize();
    self.#initScene(image);
    self.#initCamera();
    self.#initRenderer(target);
    self.#initControls(OrbitControls);
    self.rendering();
  }

  /**
   * @param {string} image
   */
  #initScene(image) {
    this.scene = new this.three.Scene();
    const geometry = new this.three.SphereGeometry(5, 32, 32);
    geometry.scale(-1, 1, 1);

    const material = new this.three.MeshBasicMaterial({
      map: new this.three.TextureLoader().load(image)
    });

    const sphere = new this.three.Mesh(geometry, material);
    this.scene.add(sphere);
  }

  #initCamera() {
    this.camera = new this.three.PerspectiveCamera(90, this.width / this.height, 1, 1000);
    this.camera.position.set(0, 0, 0.1);
  }

  /**
   * @param {HTMLElement} target
   */
  #initRenderer(target) {
    this.renderer = new this.three.WebGLRenderer();
    this.renderer.setSize(this.width, this.height);
    this.renderer.domElement.addEventListener('wheel', () => { });
    target.appendChild(this.renderer.domElement);
    this.renderer.render(this.scene, this.camera);
  }

  #initControls(OrbitControls) {
    this.controls = new OrbitControls(this.camera, this.renderer.domElement);
    this.controls.mouseButtons = {
      LEFT: this.three.MOUSE.ROTATE,
      MIDDLE: this.three.MOUSE.ROTATE,  // Default: this.three.MOUSE.DOLLY,
      RIGHT: this.three.MOUSE.ROTATE    // Default: this.three.MOUSE.PAN
    };
    this.controls.touches = {
      ONE: this.three.TOUCH.ROTATE,
      TWO: this.three.TOUCH.ROTATE  // Default: this.three.TOUCH.DOLLY_PAN
    };

    this.controls.rotateSpeed = -0.2;
    this.controls.maxDistance = this.controls.minDistance = 4;

    // this.controls.enableDamping = true;
    // this.controls.dampingFactor = 0.05;
  }

  rendering() {
    const self = this;

    function update() {
      requestAnimationFrame(update);
      window.addEventListener('resize', onWindowResize, false);
      self.renderer.render(self.scene, self.camera);
      self.controls.update();
    }

    function onWindowResize() {
      self.updateWindowSize();
      self.camera.aspect = self.width / self.height;
      self.camera.updateProjectionMatrix();
      self.renderer.setSize(self.width, self.height);
    }

    update();
  }

  updateWindowSize() {
    this.width = window.innerWidth - 20;
    this.height = window.innerHeight - 20;
  }
}
