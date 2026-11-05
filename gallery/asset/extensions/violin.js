/*
* Licensed to the Apache Software Foundation (ASF) under one
* or more contributor license agreements.  See the NOTICE file
* distributed with this work for additional information
* regarding copyright ownership.  The ASF licenses this file
* to you under the Apache License, Version 2.0 (the
* "License"); you may not use this file except in compliance
* with the License.  You may obtain a copy of the License at
*
*   http://www.apache.org/licenses/LICENSE-2.0
*
* Unless required by applicable law or agreed to in writing,
* software distributed under the License is distributed on an
* "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
* KIND, either express or implied.  See the License for the
* specific language governing permissions and limitations
* under the License.
*/

(function (global, factory) {
    typeof exports === 'object' && typeof module !== 'undefined' ? module.exports = factory() :
    typeof define === 'function' && define.amd ? define(factory) :
    (global = typeof globalThis !== 'undefined' ? globalThis : global || self, global.violinCustomSeriesInstaller = factory());
})(this, (function () { 'use strict';

    function epanechnikovKernel(u) {
        return Math.abs(u) <= 1 ? 0.75 * (1 - u * u) : 0;
    }
    function kernelDensityEstimator(kernel, bandwidth, data) {
        return function (x) {
            var sum = 0;
            for (var i = 0; i < data.length; i++) {
                sum += kernel((x - data[i]) / bandwidth);
            }
            return sum / (data.length * bandwidth);
        };
    }
    var renderItem = function (params, api) {
        var violins = {};
        if (params.context.violins == null) {
            params.context.violins = [];
            violins = params.context.violins;
            var cnt = params.dataInsideLength;
            for (var i_1 = 0; i_1 < cnt; ++i_1) {
                var xIndex = api.value(0, i_1);
                if (violins[xIndex] == null) {
                    violins[xIndex] = {
                        firstDataIndex: i_1,
                        data: [],
                    };
                }
                violins[xIndex].data.push(api.value(1, i_1));
            }
        }
        else {
            violins = params.context.violins;
        }
        params.itemPayload.symbolSize;
        var xValue = api.value(0);
        var yValue = api.value(1);
        var coord = api.coord([xValue, yValue]);
        var bandWidthScale = params.itemPayload.bandWidthScale;
        var bandWidth = (api.coord([1, 0])[0] - api.coord([0, 0])[0]) *
            (bandWidthScale == null ? 1 : bandWidthScale);
        var violin = violins[xValue];
        var violinPath = null;
        if (violin && violin.firstDataIndex === params.dataIndexInside) {
            var kde_1 = kernelDensityEstimator(epanechnikovKernel, 1, violin.data);
            var binCount = params.itemPayload.binCount || 100;
            var xRange = [];
            for (var i = 0; i < binCount; i++) {
                xRange.push(i * (10 / (binCount - 1)));
            }
            var density = xRange.map(function (x) { return [x, kde_1(x)]; });
            var epsilonDensity = 0.001;
            var polylines_1 = [];
            var points_1 = [];
            var pushToPolylines = function () {
                if (points_1.length > 1) {
                    for (var j = points_1.length - 1; j >= 0; --j) {
                        points_1.push([coord[0] * 2 - points_1[j][0], points_1[j][1]]);
                    }
                    var areaOpacity = params.itemPayload.areaOpacity;
                    polylines_1.push({
                        type: 'polygon',
                        shape: {
                            points: points_1.slice(),
                        },
                        style: {
                            fill: api.visual('color'),
                            opacity: areaOpacity == null ? 0.5 : areaOpacity,
                        },
                    });
                }
                points_1.length = 0;
            };
            for (var i_2 = 0; i_2 < density.length; ++i_2) {
                var coord_1 = api.coord([xValue, density[i_2][0]]);
                if (density[i_2][1] < epsilonDensity) {
                    pushToPolylines();
                    continue;
                }
                points_1.push([coord_1[0] + (bandWidth / 2) * density[i_2][1], coord_1[1]]);
            }
            pushToPolylines();
            violinPath = {
                type: 'group',
                children: polylines_1,
                silent: true,
            };
        }
        return violinPath;
    };
    var index = {
        install: function (registers) {
            registers.registerCustomSeries('violin', renderItem);
        },
    };

    return index;

}));
