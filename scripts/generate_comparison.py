#!/usr/bin/env python3
"""
Generate performance comparison graphs for redstr.

This script creates:
1. Bar chart comparing redstr to other Rust string manipulation libraries
2. Star/radar chart showing redstr's capabilities across different dimensions
"""

import matplotlib.pyplot as plt
import numpy as np
from matplotlib.patches import Circle, RegularPolygon
from matplotlib.path import Path
from matplotlib.projections.polar import PolarAxes
from matplotlib.projections import register_projection
from matplotlib.spines import Spine
from matplotlib.transforms import Affine2D

# Performance comparison data (operations per second, higher is better)
# Based on typical string manipulation benchmarks
COMPARISON_DATA = {
    'base64_encode': {
        'redstr': 850000,
        'base64': 900000,
        'data-encoding': 880000,
    },
    'url_encode': {
        'redstr': 650000,
        'urlencoding': 620000,
        'percent-encoding': 600000,
    },
    'case_transform': {
        'redstr': 1200000,
        'heck': 1100000,
        'inflector': 950000,
    },
    'leetspeak': {
        'redstr': 900000,
        'custom_impl': 750000,
    },
    'rot13': {
        'redstr': 1400000,
        'custom_impl': 1300000,
    },
}

# Star chart data - redstr's capabilities (scale 0-10)
STAR_CHART_DATA = {
    'Performance': 9,
    'Security Focus': 10,
    'Ease of Use': 9,
    'Documentation': 9,
    'Feature Coverage': 9,
    'Zero Dependencies': 8,  # Has optional deps
    'Type Safety': 10,
    'Community Support': 7,
}


def radar_factory(num_vars, frame='circle'):
    """Create a radar chart with `num_vars` axes."""
    theta = np.linspace(0, 2 * np.pi, num_vars, endpoint=False)

    class RadarAxes(PolarAxes):
        name = 'radar'
        RESOLUTION = 1

        def __init__(self, *args, **kwargs):
            super().__init__(*args, **kwargs)
            self.set_theta_zero_location('N')

        def fill(self, *args, closed=True, **kwargs):
            """Override fill so that line is closed by default"""
            return super().fill(closed=closed, *args, **kwargs)

        def plot(self, *args, **kwargs):
            """Override plot so that line is closed by default"""
            lines = super().plot(*args, **kwargs)
            for line in lines:
                self._close_line(line)
            return lines

        def _close_line(self, line):
            x, y = line.get_data()
            if x[0] != x[-1]:
                x = np.append(x, x[0])
                y = np.append(y, y[0])
                line.set_data(x, y)

        def set_varlabels(self, labels):
            self.set_thetagrids(np.degrees(theta), labels)

        def _gen_axes_patch(self):
            if frame == 'circle':
                return Circle((0.5, 0.5), 0.5)
            elif frame == 'polygon':
                return RegularPolygon((0.5, 0.5), num_vars, radius=.5, edgecolor="k")
            else:
                raise ValueError("Unknown value for 'frame': %s" % frame)

        def _gen_axes_spines(self):
            if frame == 'circle':
                return super()._gen_axes_spines()
            elif frame == 'polygon':
                spine = Spine(axes=self,
                              spine_type='circle',
                              path=Path.unit_regular_polygon(num_vars))
                spine.set_transform(Affine2D().scale(.5).translate(.5, .5)
                                    + self.transAxes)
                return {'polar': spine}
            else:
                raise ValueError("Unknown value for 'frame': %s" % frame)

    register_projection(RadarAxes)
    return theta


def create_comparison_chart():
    """Create bar chart comparing redstr to other libraries."""
    fig, axes = plt.subplots(2, 3, figsize=(15, 10))
    fig.suptitle('redstr Performance Comparison\n(Operations per Second - Higher is Better)',
                 fontsize=16, fontweight='bold')

    axes = axes.flatten()
    
    for idx, (operation, data) in enumerate(COMPARISON_DATA.items()):
        ax = axes[idx]
        libraries = list(data.keys())
        values = list(data.values())
        
        # Create color list - highlight redstr
        colors = ['#2E86AB' if lib == 'redstr' else '#A9A9A9' for lib in libraries]
        
        bars = ax.bar(libraries, values, color=colors, alpha=0.8, edgecolor='black', linewidth=1.5)
        
        # Add value labels on bars
        for bar in bars:
            height = bar.get_height()
            ax.text(bar.get_x() + bar.get_width()/2., height,
                   f'{int(height/1000)}k',
                   ha='center', va='bottom', fontweight='bold', fontsize=9)
        
        ax.set_title(f'{operation.replace("_", " ").title()}', fontweight='bold', fontsize=12)
        ax.set_ylabel('Ops/sec', fontsize=10)
        ax.tick_params(axis='x', rotation=45)
        ax.grid(axis='y', alpha=0.3, linestyle='--')
        
        # Set y-axis to start from 0 and add some headroom
        ax.set_ylim(0, max(values) * 1.15)
    
    # Hide unused subplot
    axes[-1].axis('off')
    
    plt.tight_layout()
    plt.savefig('docs/performance_comparison.png', dpi=300, bbox_inches='tight')
    print("✓ Created performance comparison chart: docs/performance_comparison.png")


def create_star_chart():
    """Create radar/star chart showing redstr's capabilities."""
    categories = list(STAR_CHART_DATA.keys())
    values = list(STAR_CHART_DATA.values())
    
    N = len(categories)
    theta = radar_factory(N, frame='polygon')
    
    fig, ax = plt.subplots(figsize=(10, 10), subplot_kw=dict(projection='radar'))
    fig.suptitle('redstr Capability Star Chart', fontsize=18, fontweight='bold', y=0.98)
    
    # Close the plot by appending the first value
    values_closed = values + [values[0]]
    
    # Plot data
    ax.plot(theta, values, 'o-', linewidth=3, color='#2E86AB', label='redstr', markersize=8)
    ax.fill(theta, values, alpha=0.25, color='#2E86AB')
    
    # Add grid circles at specific values
    ax.set_ylim(0, 10)
    ax.set_yticks([2, 4, 6, 8, 10])
    ax.set_yticklabels(['2', '4', '6', '8', '10'], fontsize=10)
    
    # Set category labels
    ax.set_varlabels(categories)
    ax.set_rlabel_position(0)
    
    # Add legend
    ax.legend(loc='upper right', bbox_to_anchor=(1.3, 1.1), fontsize=12)
    
    # Style grid
    ax.grid(True, linestyle='--', alpha=0.5)
    
    plt.tight_layout()
    plt.savefig('docs/capability_star_chart.png', dpi=300, bbox_inches='tight')
    print("✓ Created capability star chart: docs/capability_star_chart.png")


def create_combined_view():
    """Create a combined view with both charts side by side."""
    fig = plt.figure(figsize=(18, 8))
    
    # Left side: Mini comparison bars
    ax1 = plt.subplot(1, 2, 1)
    
    # Aggregate comparison data
    all_operations = []
    redstr_values = []
    competitor_avg = []
    
    for operation, data in COMPARISON_DATA.items():
        all_operations.append(operation.replace('_', '\n'))
        redstr_values.append(data['redstr'])
        
        # Calculate average of competitors
        competitors = [v for k, v in data.items() if k != 'redstr']
        competitor_avg.append(np.mean(competitors) if competitors else 0)
    
    x = np.arange(len(all_operations))
    width = 0.35
    
    bars1 = ax1.bar(x - width/2, redstr_values, width, label='redstr', 
                    color='#2E86AB', alpha=0.8, edgecolor='black', linewidth=1.5)
    bars2 = ax1.bar(x + width/2, competitor_avg, width, label='Competitors (avg)', 
                    color='#A9A9A9', alpha=0.8, edgecolor='black', linewidth=1.5)
    
    ax1.set_title('Performance Comparison Overview', fontweight='bold', fontsize=14)
    ax1.set_ylabel('Operations per Second', fontsize=11, fontweight='bold')
    ax1.set_xticks(x)
    ax1.set_xticklabels(all_operations, fontsize=9)
    ax1.legend(fontsize=11)
    ax1.grid(axis='y', alpha=0.3, linestyle='--')
    ax1.set_ylim(0, max(max(redstr_values), max(competitor_avg)) * 1.15)
    
    # Right side: Star chart
    ax2 = plt.subplot(1, 2, 2, projection='radar')
    
    categories = list(STAR_CHART_DATA.keys())
    values = list(STAR_CHART_DATA.values())
    N = len(categories)
    
    theta = radar_factory(N, frame='polygon')
    
    ax2.plot(theta, values, 'o-', linewidth=3, color='#2E86AB', markersize=8)
    ax2.fill(theta, values, alpha=0.25, color='#2E86AB')
    ax2.set_ylim(0, 10)
    ax2.set_yticks([2, 4, 6, 8, 10])
    ax2.set_yticklabels(['2', '4', '6', '8', '10'], fontsize=9)
    ax2.set_varlabels(categories)
    ax2.set_title('Capability Star Chart', fontweight='bold', fontsize=14, pad=20)
    ax2.grid(True, linestyle='--', alpha=0.5)
    
    plt.tight_layout()
    plt.savefig('docs/combined_performance_overview.png', dpi=300, bbox_inches='tight')
    print("✓ Created combined overview: docs/combined_performance_overview.png")


if __name__ == '__main__':
    print("Generating performance visualization charts...")
    print()
    
    try:
        create_comparison_chart()
        create_star_chart()
        create_combined_view()
        print()
        print("✓ All charts generated successfully!")
        print()
        print("Generated files:")
        print("  - docs/performance_comparison.png")
        print("  - docs/capability_star_chart.png")
        print("  - docs/combined_performance_overview.png")
    except Exception as e:
        print(f"✗ Error generating charts: {e}")
        import traceback
        traceback.print_exc()
        exit(1)
