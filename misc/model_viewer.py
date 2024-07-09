import numpy as np
import ujson
import pyvista as pv
from matplotlib import colormaps as cm

class ViewerEngine:
    def __init__(self, mesh, model, x_axis, y_axis, z_axis):
        self.model = model
        self.output = mesh  # Expected PyVista mesh type
        self.x_axis = x_axis
        self.y_axis = y_axis
        self.z_axis = z_axis
        self.kwargs = {
            'x_axis': len(model[0][0]),
            'y_axis': len(model[0]),
            'z_axis': len(model),
        }

    def __call__(self, param, value):
        self.kwargs[param] = value
        if param == "z_axis":
            self.update_z_axis(value)
        elif param == "y_axis":
            self.update_y_axis(value)
        elif param == "x_axis":
            self.update_x_axis(value)

    def value_to_num(self, value, axis):
        def find_elem(value, axis):
            min_bound = 0
            max_bound = len(axis)
            while True:
                if max_bound - min_bound == 1:
                    return max_bound 
                if value == axis[(max_bound + min_bound) // 2]:
                    return (max_bound + min_bound) // 2 + 1 
                elif value > axis[(max_bound + min_bound) // 2]:
                    min_bound = (max_bound + min_bound) // 2
                else:
                    max_bound = (max_bound + min_bound) // 2

        match axis:
            case "y_axis":
                return find_elem(value, self.y_axis)
            case "x_axis":
                return find_elem(value, self.x_axis)
            case "z_axis":
                return find_elem(value, self.z_axis)
        return value


    def update_z_axis(self, value):
        new_model = self.model[:self.value_to_num(value, "z_axis")]
        mesh = pv.ImageData()
        mesh.dimensions = np.array((len(new_model[0][0]), len(new_model[0]), len(new_model))) + 1
        mesh.origin = (0, 0, 0)
        mesh.spacing = (1, 1, 1)
        mesh.cell_data["values"] = new_model.flatten()
        self.output.copy_from(mesh)

    def update_y_axis(self, value):
        new_model = self.model[:, :self.value_to_num(value, "y_axis"), :]
        mesh = pv.ImageData()
        mesh.dimensions = np.array((len(new_model[0][0]), len(new_model[0]), len(new_model))) + 1
        mesh.origin = (0, 0, 0)
        mesh.spacing = (1, 1, 1)
        mesh.cell_data["values"] = new_model.flatten()
        self.output.copy_from(mesh)

    def update_x_axis(self, value):
        new_model = self.model[:, :, :self.value_to_num(value, "x_axis")]
        mesh = pv.ImageData()
        mesh.dimensions = np.array((len(new_model[0][0]), len(new_model[0]), len(new_model))) + 1
        mesh.origin = (0, 0, 0)
        mesh.spacing = (1, 1, 1)
        mesh.cell_data["values"] = new_model.flatten()
        self.output.copy_from(mesh)

def main():
    try:
        model_file_source = open("../my_model.json")
    except FileNotFoundError:
        model_file_source = open("./my_model.json")
    print("loading", model_file_source)
    model_file = ujson.load(model_file_source)

    print("Json loaded")
    del(model_file_source)

    target = "model_mask"
    x_s = len(model_file[target])
    y_s = len(model_file[target][0]["x0"])
    z_s = len(model_file[target][0]["x0"][0]["y0"])
    print(f"{target} size(x, y, z): ",x_s, y_s, z_s)

    loaded_model = np.empty(dtype=int, shape=(x_s, y_s, z_s))
    for i, e in enumerate(model_file["model_mask"]):
        for j, ee in enumerate(e[f"x{i}"]):
            # for k in ee[f"y{j}"]:
            #     loaded_model[i][j][k] = int(k)

            loaded_model[i][j] = np.array([int(k) for k in ee[f"y{j}"]])

    x_axis = np.array([float(i) for i in model_file["output_axes"]["x_ax"]])
    y_axis = np.array([float(i) for i in model_file["output_axes"]["y_ax"]])
    z_axis = np.array([float(i) for i in model_file["output_axes"]["z_ax"]])

    print("JSON converted")
    del(model_file)

    model = np.empty(dtype=int, shape=(z_s, y_s, x_s))
    for i in range(z_s):
        now_i = z_s - i - 1
        for j in range(y_s):
            for k in range(x_s):
                model[now_i][j][k] = loaded_model[k][j][i]

    print("JSON restructured")
    del(loaded_model)

    mesh = pv.ImageData()
    mesh.dimensions = np.array((len(model[0][0]),len(model[0]), len(model))) + 1
    mesh.origin = (0, 0, 0)
    mesh.spacing = (1, 1, 1)
    mesh.cell_data["values"] = model.flatten()
    # mesh.cell_data["values"] = np.arange(mesh.n_cells)

    p = pv.Plotter()
    colors = cm.get_cmap("Wistia")
    p.add_mesh(mesh, opacity=1, show_edges=True, cmap=colors)
    p.show_bounds(axes_ranges=[x_axis[0], x_axis[-1], y_axis[0], y_axis[-1], z_axis[0], z_axis[-1]], color="black", location="all", xtitle="X ax", ytitle="Y ax")


    engine = ViewerEngine(mesh, model.copy(), x_axis, y_axis, z_axis)

    p.add_slider_widget(
        callback=lambda value: engine('z_axis', value),
        rng=[1, len(model)],
        value=len(model),
        title="Z axis",
        pointa=(0.025, 0.1),
        pointb=(0.31, 0.1),
        style="modern"
    )
    p.add_slider_widget(
        callback=lambda value: engine('y_axis', value),
        rng=[y_axis[0], y_axis[-1]],
        value=y_axis[-1],
        title="Y axis",
        pointa=(0.025, 0.2),
        pointb=(0.31, 0.2),
        style="modern"
    )
    p.add_slider_widget(
        callback=lambda value: engine('x_axis', value),
        rng=[x_axis[0], x_axis[-1]],
        value=x_axis[-1],
        title="X axis",
        pointa=(0.025, 0.3),
        pointb=(0.31, 0.3),
        style="modern"
    )

    p.show()

if __name__ == "__main__":
    main()
