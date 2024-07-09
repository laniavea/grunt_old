import json
import numpy as np
import matplotlib.pyplot as plt

def main():
    try:
        model_file = open("../my_model2D.json")
    except FileNotFoundError:
        model_file = open("./my_model2D.json")

    print("Loading 2D", model_file)
    model_file = json.load(model_file)

    model = []
    for i, e in enumerate(model_file["model_mask"]):
        model.append([])
        for j in e[f"x{i}"]:
            model[-1].append(int(j))

    x_axis = [float(i) for i in model_file["output_axes"]["x_ax"]]
    z_axis = [float(i) for i in model_file["output_axes"]["z_ax"]]

    z_s = len(model[0])
    x_s = len(model)

    print(x_axis)
    print(z_axis)

    model = np.array(model).T

    fig, ax = plt.subplots(figsize=(10, 10))
    ax.imshow(model, aspect='auto', cmap="Wistia")

    ax.set_xticks(x_axis)
    ax.set_yticks(z_axis[::10])
    ax.set_yticklabels(np.array(z_axis[::10]) / 100)

    ax.set_title("2D model")

    plt.show()

if __name__ == "__main__":
    main()

