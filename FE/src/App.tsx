import { useEffect, useMemo, useRef, useState } from 'react';

type Action = 0 | 1 | 2 | 3 | 4 | 5;
type TickRecord = [energy: number, action: Action, x: number, y: number];
type GrassDeath = [tick: number, x: number, y: number];

type AnimalStart = {
  energy: number;
  action: Action;
  pos_x: number;
  pos_y: number;
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
  0: 'walk left',
  1: 'walk right',
  2: 'walk up',
  3: 'walk down',
  4: 'eat',
  5: 'none'
};

const PLAY_MS_PER_STEP = 250;
const CELL_SIZE = 14;

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
      check(record[2], record[3]);
    });
  });

  generation.herbivore_records.forEach((records) => {
    records.forEach((record) => {
      check(record[2], record[3]);
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

    const dx = Math.abs(record[2] - x);
    const dy = Math.abs(record[3] - y);
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

    const dx = Math.abs(record[2] - x);
    const dy = Math.abs(record[3] - y);
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
  const [simulation, setSimulation] = useState<SimulationResponse | null>(null);
  const [generationIndex, setGenerationIndex] = useState(0);
  const [tick, setTick] = useState(0);
  const [playing, setPlaying] = useState(false);
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

    const handle = window.setInterval(() => {
      setTick((previous) => {
        if (previous >= tickMax) {
          return previous;
        }
        return previous + 1;
      });
    }, PLAY_MS_PER_STEP);

    return () => window.clearInterval(handle);
  }, [playing, tickMax]);

  useEffect(() => {
    if (tick >= tickMax) {
      setPlaying(false);
    }
  }, [tick, tickMax]);

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
        record[2],
        record[3],
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
        record[2],
        record[3],
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

    if (tick >= tickMax) {
      setTick(0);
    }

    setPlaying((current) => !current);
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
          <button onClick={stepBack} disabled={!generation || tick === 0}>
            Step Back
          </button>
          <button onClick={togglePlay} disabled={!generation}>
            {playing ? 'Pause' : 'Play'}
          </button>
          <button onClick={stepForward} disabled={!generation || tick >= tickMax}>
            Step Forward
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
          Tick: {tick} / {tickMax} | Speed: 4 ticks/sec
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
                <dd>{selectedRecord[0]}</dd>
              </div>
              <div>
                <dt>Action</dt>
                <dd>{ACTION_NAMES[selectedRecord[1]]}</dd>
              </div>
              <div>
                <dt>X</dt>
                <dd>{selectedRecord[2]}</dd>
              </div>
              <div>
                <dt>Y</dt>
                <dd>{selectedRecord[3]}</dd>
              </div>
            </dl>
          )}
        </aside>
      </section>
    </main>
  );
}
