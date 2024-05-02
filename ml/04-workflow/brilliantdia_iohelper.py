#!/usr/bin/env python
# coding: utf-8

import pandas as pd
import numpy as np


def readAsClass(fileName, numClasses=2):
    data = pd.read_csv(fileName)
    boundaries = np.logspace(2, 5, numClasses+1, base=10)
    boundaries[0] = 0
    boundaries[-1] = np.inf
    classes = pd.cut(data.price, boundaries, labels=range(numClasses)).astype('int')
    data['priceClass'] = classes
    return data.drop('price', axis=1), boundaries


if __name__ == '__main__':
    data, boundaries = readAsClass('brilliantdia-train.csv', 3)
    print(boundaries)
    print(data.head())

