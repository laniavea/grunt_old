import numpy as np
import json
import matplotlib.pyplot as plt

def flatten_to_zyx(model):
    size_x = len(model)
    size_y = len(model[0])
    size_z = len(model[0][0])
    layer_count = size_x * size_y
    size_count = size_x * size_y * size_z

    array_flatten = np.zeros(size_count)
    for i in range(0, size_count):
        array_flatten[i] = model[i%size_x][(i%layer_count)//size_x][i//layer_count]

    return array_flatten

def main():
    try:
        model_file = open("../my_model.json")
    except FileNotFoundError:
         model_file = open("./my_model.json")
    model_file = json.load(model_file)

    model = []
    for i, e in enumerate(model_file["model"]):
        model.append([])
        for j, ee in enumerate(e[f"x{i}"]):
            model[i].append([])
            for k in ee[f"y{j}"]:
                model[i][j].append(int(k))

    z_s = len(model[0][0])
    y_s = len(model[0])
    x_s = len(model)

    model = flatten_to_zyx(np.array(model))
    model = np.array(model.reshape(z_s, y_s, x_s))

    z, y, x = np.indices(np.array(model.shape) + 1).astype(int)

    ax = plt.figure().add_subplot(111, projection='3d')

    colors = plt.cm.plasma(model)

    ax.voxels(x, y, z, model, facecolors=colors, alpha=0.8)

    ax.invert_zaxis()
    ax.set_xlabel("X_axis")
    ax.set_ylabel("Y_axis")
    ax.set_zlabel("Z_axis(depth)")
    plt.show()


if __name__ == "__main__":
    main()

