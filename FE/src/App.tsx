import { useEffect, useMemo, useRef, useState } from 'react';

type Action = 'WalkLeft' | 'WalkRight' | 'WalkUp' | 'WalkDown' | 'Eat' | 'None';
type TickRecord = {
  action: Action;
  energy: number;
  pos_x: number;
  pos_y: number;
};
type GrassDeath = [tick: number, x: number, y: number];
type Neuron = {
  constants: number[];
  output: number;
};
type NeuronGrid = Neuron[][];

type AnimalStart = {
  energy: number;
  action: Action;
  pos_x: number;
  pos_y: number;
  neurons?: NeuronGrid;
};

type Generation = {
  carnivores_at_start: AnimalStart[];
  herbivores_at_start: AnimalStart[];
  grass_at_start: number[][];
  carnivore_records: TickRecord[][];
  herbivore_records: TickRecord[][];
  dead_grass: GrassDeath[];
};

type SimulationResponse = {
  generations: Generation[];
};

type Species = 'carnivore' | 'herbivore';

type SelectedAnimal = {
  species: Species;
  index: number;
};

type AnimalHit = {
  species: Species;
  index: number;
  distance: number;
};

const ACTION_NAMES: Record<Action, string> = {
  WalkLeft: 'walk left',
  WalkRight: 'walk right',
  WalkUp: 'walk up',
  WalkDown: 'walk down',
  Eat: 'eat',
  None: 'none'
};

const OUTPUT_LABELS = ['WalkLeft', 'WalkRight', 'WalkUp', 'WalkDown', 'Eat', 'None'];

const DEFAULT_SPEED = 4;
const MIN_SPEED = 2;
const CELL_SIZE = 14;

const clamp = (value: number, min: number, max: number): number =>
  Math.min(max, Math.max(min, value));

const constantToColor = (value: number): string => {
  const t = (clamp(value, -1, 1) + 1) / 2;
  const red = Math.round(230 * (1 - t));
  const blue = Math.round(230 * t);
  const green = 40;

  return `rgb(${red}, ${green}, ${blue})`;
};

const getSpeedIncrement = (speedAbs: number): number => {
  if (speedAbs >= 1000) {
    return 100;
  }
  if (speedAbs >= 100) {
    return 10;
  }
  if (speedAbs >= 12) {
    return 4;
  }

  return 2;
};

const getTickMax = (generation: Generation): number => {
  const carnivoreTicks = generation.carnivore_records.map((records) => records.length);
  const herbivoreTicks = generation.herbivore_records.map((records) => records.length);
  const deathTicks = generation.dead_grass.map((g) => g[0] + 1);
  const allLengths = [...carnivoreTicks, ...herbivoreTicks, ...deathTicks];

  if (allLengths.length === 0) {
    return 0;
  }

  return Math.max(...allLengths) - 1;
};

const getGridBounds = (generation: Generation): { width: number; height: number } => {
  let maxX = 0;
  let maxY = 0;

  const check = (x: number, y: number) => {
    if (x > maxX) {
      maxX = x;
    }
    if (y > maxY) {
      maxY = y;
    }
  };

  generation.grass_at_start.forEach((grass) => {
    if (grass.length >= 2) {
      check(grass[0], grass[1]);
    }
  });

  generation.dead_grass.forEach((grass) => {
    check(grass[1], grass[2]);
  });

  generation.carnivores_at_start.forEach((animal) => {
    check(animal.pos_x, animal.pos_y);
  });

  generation.herbivores_at_start.forEach((animal) => {
    check(animal.pos_x, animal.pos_y);
  });

  generation.carnivore_records.forEach((records) => {
    records.forEach((record) => {
      check(record.pos_x, record.pos_y);
    });
  });

  generation.herbivore_records.forEach((records) => {
    records.forEach((record) => {
      check(record.pos_x, record.pos_y);
    });
  });

  return {
    width: maxX + 1,
    height: maxY + 1
  };
};

const getAnimalRecord = (
  generation: Generation,
  species: Species,
  index: number,
  tick: number
): TickRecord | null => {
  const records =
    species === 'carnivore'
      ? generation.carnivore_records[index]
      : generation.herbivore_records[index];

  if (!records || tick < 0 || tick >= records.length) {
    return null;
  }

  return records[tick];
};

const findNearestAnimalAtTick = (
  generation: Generation,
  tick: number,
  x: number,
  y: number
): SelectedAnimal | null => {
  let best: AnimalHit | null = null;

  generation.carnivore_records.forEach((records, index) => {
    const record = records[tick];
    if (!record) {
      return;
    }

    const dx = Math.abs(record.pos_x - x);
    const dy = Math.abs(record.pos_y - y);
    const distance = Math.max(dx, dy);
    if (distance > 1) {
      return;
    }

    if (!best || distance < best.distance) {
      best = { species: 'carnivore', index, distance };
    }
  });

  generation.herbivore_records.forEach((records, index) => {
    const record = records[tick];
    if (!record) {
      return;
    }

    const dx = Math.abs(record.pos_x - x);
    const dy = Math.abs(record.pos_y - y);
    const distance = Math.max(dx, dy);
    if (distance > 1) {
      return;
    }

    if (!best || distance < best.distance) {
      best = { species: 'herbivore', index, distance };
    }
  });

  if (!best) {
    return null;
  }

  return { species: best.species, index: best.index };
};

const drawPixel = (
  ctx: CanvasRenderingContext2D,
  x: number,
  y: number,
  color: string,
  highlighted: boolean,
  width: number,
  height: number
) => {
  const px = x * CELL_SIZE;
  const py = y * CELL_SIZE;

  if (!highlighted) {
    ctx.fillStyle = color;
    ctx.fillRect(px, py, CELL_SIZE, CELL_SIZE);
    return;
  }

  ctx.fillStyle = color;
  ctx.fillRect(px, py, CELL_SIZE, CELL_SIZE);

  if (x + 1 < width) {
    ctx.fillRect((x + 1) * CELL_SIZE, py, CELL_SIZE, CELL_SIZE);
  }
  if (y + 1 < height) {
    ctx.fillRect(px, (y + 1) * CELL_SIZE, CELL_SIZE, CELL_SIZE);
  }
  if (x + 1 < width && y + 1 < height) {
    ctx.fillRect((x + 1) * CELL_SIZE, (y + 1) * CELL_SIZE, CELL_SIZE, CELL_SIZE);
  }
};

export default function App() {
  const canvasRef = useRef<HTMLCanvasElement | null>(null);
  const neuronCanvasRef = useRef<HTMLCanvasElement | null>(null);
  const [simulation, setSimulation] = useState<SimulationResponse | null>(null);
  const [generationIndex, setGenerationIndex] = useState(0);
  const [tick, setTick] = useState(0);
  const [playing, setPlaying] = useState(false);
  const [playbackSpeed, setPlaybackSpeed] = useState(DEFAULT_SPEED);
  const [selectedAnimal, setSelectedAnimal] = useState<SelectedAnimal | null>(null);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const generation = simulation?.generations[generationIndex] ?? null;

  const gridSize = useMemo(() => {
    if (!generation) {
      return { width: 20, height: 10 };
    }

    return getGridBounds(generation);
  }, [generation]);

  const tickMax = useMemo(() => {
    if (!generation) {
      return 0;
    }

    return getTickMax(generation);
  }, [generation]);

  useEffect(() => {
    if (!playing) {
      return;
    }

    const speedAbs = Math.max(Math.abs(playbackSpeed), MIN_SPEED);
    const direction = playbackSpeed < 0 ? -1 : 1;
    const intervalMs = 1000 / speedAbs;

    const handle = window.setInterval(() => {
      setTick((previous) => {
        const next = previous + direction;

        if (next > tickMax) {
          return previous;
        }
        if (next < 0) {
          return previous;
        }

        return next;
      });
    }, intervalMs);

    return () => window.clearInterval(handle);
  }, [playbackSpeed, playing, tickMax]);

  useEffect(() => {
    if (!playing) {
      return;
    }

    if (playbackSpeed > 0 && tick >= tickMax) {
      setPlaying(false);
      return;
    }

    if (playbackSpeed < 0 && tick <= 0) {
      setPlaying(false);
    }
  }, [playbackSpeed, playing, tick, tickMax]);

  useEffect(() => {
    const canvas = canvasRef.current;
    if (!canvas || !generation) {
      return;
    }

    canvas.width = gridSize.width * CELL_SIZE;
    canvas.height = gridSize.height * CELL_SIZE;

    const ctx = canvas.getContext('2d');
    if (!ctx) {
      return;
    }

    ctx.fillStyle = '#0e1418';
    ctx.fillRect(0, 0, canvas.width, canvas.height);

    ctx.fillStyle = '#1d2933';
    for (let x = 0; x < gridSize.width; x += 1) {
      for (let y = 0; y < gridSize.height; y += 1) {
        ctx.fillRect(x * CELL_SIZE, y * CELL_SIZE, CELL_SIZE - 1, CELL_SIZE - 1);
      }
    }

    const grassAlive = new Set(generation.grass_at_start.map((g) => `${g[0]},${g[1]}`));
    generation.dead_grass.forEach((g) => {
      if (g[0] <= tick) {
        grassAlive.delete(`${g[1]},${g[2]}`);
      }
    });

    grassAlive.forEach((key) => {
      const [x, y] = key.split(',').map((v) => Number(v));
      drawPixel(ctx, x, y, '#1fbf56', false, gridSize.width, gridSize.height);
    });

    generation.carnivore_records.forEach((records, index) => {
      const record = records[tick];
      if (!record) {
        return;
      }

      const isSelected =
        selectedAnimal?.species === 'carnivore' && selectedAnimal.index === index;

      drawPixel(
        ctx,
        record.pos_x,
        record.pos_y,
        '#eb3f47',
        Boolean(isSelected),
        gridSize.width,
        gridSize.height
      );
    });

    generation.herbivore_records.forEach((records, index) => {
      const record = records[tick];
      if (!record) {
        return;
      }

      const isSelected =
        selectedAnimal?.species === 'herbivore' && selectedAnimal.index === index;

      drawPixel(
        ctx,
        record.pos_x,
        record.pos_y,
        '#4c74ff',
        Boolean(isSelected),
        gridSize.width,
        gridSize.height
      );
    });
  }, [generation, gridSize.height, gridSize.width, selectedAnimal, tick]);

  const selectedRecord = useMemo(() => {
    if (!generation || !selectedAnimal) {
      return null;
    }

    return getAnimalRecord(generation, selectedAnimal.species, selectedAnimal.index, tick);
  }, [generation, selectedAnimal, tick]);

  const selectedAnimalStart = useMemo(() => {
    if (!generation || !selectedAnimal) {
      return null;
    }

    return selectedAnimal.species === 'carnivore'
      ? generation.carnivores_at_start[selectedAnimal.index] ?? null
      : generation.herbivores_at_start[selectedAnimal.index] ?? null;
  }, [generation, selectedAnimal]);

  const selectedNeurons = selectedAnimalStart?.neurons ?? null;

  useEffect(() => {
    const canvas = neuronCanvasRef.current;
    if (!canvas) {
      return;
    }

    const ctx = canvas.getContext('2d');
    if (!ctx) {
      return;
    }

    if (!selectedNeurons || selectedNeurons.length === 0) {
      canvas.width = 20;
      canvas.height = 20;
      ctx.clearRect(0, 0, canvas.width, canvas.height);
      return;
    }

    const hiddenRows = selectedNeurons.slice(0, -1);
    const outputRow = selectedNeurons[selectedNeurons.length - 1] ?? [];
    const rows = hiddenRows.length;
    const cols = hiddenRows.reduce((max, row) => Math.max(max, row.length), 0);
    const maxConstants = selectedNeurons.reduce((max, row) => {
      return Math.max(
        max,
        row.reduce((innerMax, neuron) => Math.max(innerMax, neuron.constants.length), 0)
      );
    }, 0);

    const cellWidth = 20;
    const cellHeight = 80;
    const columnGap = 12;
    const rowGap = 12;
    const padding = 14;
    const outputBlockWidth = 78;
    const outputLabelHeight = 18;
    const outputRowWidth =
      outputRow.length > 0
        ? outputRow.length * outputBlockWidth + (outputRow.length - 1) * columnGap
        : 0;
    const hiddenGridWidth = cols > 0 ? cols * cellWidth + (cols - 1) * columnGap : 0;
    const hiddenGridHeight = rows > 0 ? rows * cellHeight + (rows - 1) * rowGap : 0;
    const outputRowHeight =
      outputRow.length > 0 ? cellHeight + outputLabelHeight + rowGap : 0;
    const contentWidth = Math.max(hiddenGridWidth, outputRowWidth, cellWidth);

    canvas.width = padding * 2 + contentWidth;
    canvas.height = padding * 2 + hiddenGridHeight + outputRowHeight;

    ctx.fillStyle = '#07141d';
    ctx.fillRect(0, 0, canvas.width, canvas.height);
    ctx.textAlign = 'center';
    ctx.textBaseline = 'middle';
    ctx.font = '12px "Trebuchet MS", "Segoe UI", sans-serif';

    hiddenRows.forEach((row, rowIndex) => {
      row.forEach((neuron, columnIndex) => {
        const cellX = padding + columnIndex * (cellWidth + columnGap);
        const cellY = padding + rowIndex * (cellHeight + rowGap);

        ctx.strokeStyle = '#2f6b62';
        ctx.lineWidth = 1;
        ctx.strokeRect(cellX, cellY, cellWidth, cellHeight);

        const values = neuron.constants;
        if (values.length === 0) {
          return;
        }

        const ySpacing = (cellHeight - 16) / (values.length + 1);
        values.forEach((value, valueIndex) => {
          const y = cellY + 8 + (valueIndex + 1) * ySpacing;
          ctx.strokeStyle = constantToColor(value);
          ctx.lineWidth = 4;
          ctx.beginPath();
          ctx.moveTo(cellX + 3, y);
          ctx.lineTo(cellX + cellWidth - 3, y);
          ctx.stroke();
        });
      });
    });

    if (outputRow.length > 0) {
      const outputOffsetX = padding + Math.max(0, (contentWidth - outputRowWidth) / 2);
      const outputY = padding + hiddenGridHeight + (hiddenGridHeight > 0 ? rowGap : 0);

      outputRow.forEach((neuron, outputIndex) => {
        const blockX = outputOffsetX + outputIndex * (outputBlockWidth + columnGap);
        const cellX = blockX + (outputBlockWidth - cellWidth) / 2;

        ctx.strokeStyle = '#2f6b62';
        ctx.lineWidth = 1;
        ctx.strokeRect(cellX, outputY, cellWidth, cellHeight);

        const values = neuron.constants;
        if (values.length > 0) {
          const ySpacing = (cellHeight - 16) / (values.length + 1);
          values.forEach((value, valueIndex) => {
            const y = outputY + 8 + (valueIndex + 1) * ySpacing;
            ctx.strokeStyle = constantToColor(value);
            ctx.lineWidth = 4;
            ctx.beginPath();
            ctx.moveTo(cellX + 3, y);
            ctx.lineTo(cellX + cellWidth - 3, y);
            ctx.stroke();
          });
        }

        ctx.fillStyle = '#ddf8f1';
        ctx.fillText(
          OUTPUT_LABELS[outputIndex] ?? `Output ${outputIndex}`,
          blockX + outputBlockWidth / 2,
          outputY + cellHeight + outputLabelHeight / 2 + 2
        );
      });
    }
  }, [selectedNeurons]);

  const loadSimulation = async () => {
    setLoading(true);
    setError(null);

    try {
      const response = await fetch('http://127.0.0.1:3000/simulate');
      if (!response.ok) {
        throw new Error(`HTTP ${response.status}`);
      }

      const json = (await response.json()) as SimulationResponse;
      setSimulation(json);
      setGenerationIndex(0);
      setTick(0);
      setPlaying(false);
      setPlaybackSpeed(DEFAULT_SPEED);
      setSelectedAnimal(null);
    } catch (loadError) {
      const message = loadError instanceof Error ? loadError.message : 'Unknown error';
      setError(`Could not load simulation: ${message}`);
    } finally {
      setLoading(false);
    }
  };

  const stepBack = () => {
    setPlaying(false);
    setTick((current) => (current > 0 ? current - 1 : 0));
  };

  const stepForward = () => {
    setTick((current) => (current < tickMax ? current + 1 : current));
  };

  const togglePlay = () => {
    if (!generation) {
      return;
    }

    if (playbackSpeed > 0 && tick >= tickMax) {
      setTick(0);
    }

    if (playbackSpeed < 0 && tick <= 0) {
      setTick(tickMax);
    }

    setPlaying((current) => !current);
  };

  const increaseSpeedForward = () => {
    setPlaybackSpeed((current) => {
      if (current < 0) {
        return MIN_SPEED;
      }

      return current + getSpeedIncrement(Math.abs(current));
    });
  };

  const increaseSpeedBackward = () => {
    setPlaybackSpeed((current) => {
      if (current > 0) {
        return -MIN_SPEED;
      }

      return current - getSpeedIncrement(Math.abs(current));
    });
  };

  const onCanvasClick = (event: React.MouseEvent<HTMLCanvasElement>) => {
    if (!generation) {
      return;
    }

    const canvas = event.currentTarget;
    const rect = canvas.getBoundingClientRect();
    const scaleX = canvas.width / rect.width;
    const scaleY = canvas.height / rect.height;
    const canvasX = (event.clientX - rect.left) * scaleX;
    const canvasY = (event.clientY - rect.top) * scaleY;
    const x = Math.floor(canvasX / CELL_SIZE);
    const y = Math.floor(canvasY / CELL_SIZE);

    const hit = findNearestAnimalAtTick(generation, tick, x, y);
    setSelectedAnimal(hit);
  };

  return (
    <main className="layout">
      <section className="controls">
        <h1>Evolution Simulation Viewer</h1>
        <div className="toolbar">
          <button onClick={loadSimulation} disabled={loading}>
            {loading ? 'Loading...' : 'Load Simulation'}
          </button>
          <button
            onClick={increaseSpeedBackward}
            disabled={!generation}
            aria-label="Increase speed backwards"
            title="Increase speed backwards"
          >
            ⏪
          </button>
          <button onClick={stepBack} disabled={!generation || tick === 0}>
            ⏮
          </button>
          <button onClick={togglePlay} disabled={!generation}>
            {playing ? '⏸' : '⏵'}
          </button>
          <button onClick={stepForward} disabled={!generation || tick >= tickMax}>
            ⏭
          </button>
          <button
            onClick={increaseSpeedForward}
            disabled={!generation}
            aria-label="Increase speed forward"
            title="Increase speed forward"
          >
            ⏩
          </button>
          <select
            value={generationIndex}
            disabled={!simulation || simulation.generations.length < 2}
            onChange={(event) => {
              setGenerationIndex(Number(event.target.value));
              setTick(0);
              setPlaying(false);
              setSelectedAnimal(null);
            }}
          >
            {(simulation?.generations ?? []).map((_, index) => (
              <option key={index} value={index}>
                Generation {index}
              </option>
            ))}
          </select>
        </div>

        <p className="status">
          Tick: {tick} / {tickMax} | Speed: {playbackSpeed} ticks/sec
        </p>
        {error && <p className="error">{error}</p>}
      </section>

      <section className="viewer">
        <canvas
          ref={canvasRef}
          onClick={onCanvasClick}
          className="grid"
          aria-label="Simulation grid"
        />

        <aside className="stats">
          <h2>Selected Animal</h2>
          {!selectedAnimal || !generation || !selectedRecord ? (
            <p>Click a red or blue pixel to inspect an animal.</p>
          ) : (
            <dl>
              <div>
                <dt>Generation</dt>
                <dd>{generationIndex}</dd>
              </div>
              <div>
                <dt>Species</dt>
                <dd>{selectedAnimal.species}</dd>
              </div>
              <div>
                <dt>Index</dt>
                <dd>{selectedAnimal.index}</dd>
              </div>
              <div>
                <dt>Tick</dt>
                <dd>{tick}</dd>
              </div>
              <div>
                <dt>Energy</dt>
                <dd>{selectedRecord.energy}</dd>
              </div>
              <div>
                <dt>Action</dt>
                <dd>{ACTION_NAMES[selectedRecord.action]}</dd>
              </div>
              <div>
                <dt>X</dt>
                <dd>{selectedRecord.pos_x}</dd>
              </div>
              <div>
                <dt>Y</dt>
                <dd>{selectedRecord.pos_y}</dd>
              </div>
            </dl>
          )}
        </aside>
      </section>

      <section className="neurons-panel">
        <h2>Neurons</h2>
        {!selectedAnimal || !selectedNeurons || selectedNeurons.length === 0 ? (
          <p>Select an animal to visualize its neuron constants.</p>
        ) : (
          <>
            <p>
              Grid: {selectedNeurons.reduce((max, row) => Math.max(max, row.length), 0)} columns x{' '}
              {selectedNeurons.length} rows. Constant colors: red (-1) to blue (+1).
            </p>
            <div className="neurons-scroll">
              <canvas ref={neuronCanvasRef} className="neurons-canvas" aria-label="Neuron visualization" />
            </div>
          </>
        )}
      </section>
    </main>
  );
}
