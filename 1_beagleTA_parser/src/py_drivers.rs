// Function to generate Python script for visualizations
pub fn generate_python_script(
    keywords: &[String],
    heatmap_data: &[Vec<f64>],
    file_names: &[String],
    cosine_similarities: &[f64],
    output_dir: &str,
) -> String {
    format!(
        r#"
import pandas as pd
import plotly.express as px
import numpy as np
from scipy.cluster.hierarchy import linkage, leaves_list
from scipy.spatial.distance import pdist
from sklearn.cluster import KMeans
import os
import webbrowser

# Ensure the output directory exists
os.makedirs("{output_dir}", exist_ok=True)

# Data preparation
keywords = {keywords:?}
file_names = {file_names:?}
heatmap_data = np.array({heatmap_data:?})
cosine_similarities = np.array({cosine_similarities:?})

# Create DataFrame
df = pd.DataFrame(heatmap_data, columns=keywords, index=file_names)

# Filter rows with all zero values and create a copy to avoid SettingWithCopyWarning
df_filtered = df.loc[(df != 0).any(axis=1)].copy()

if df_filtered.empty:
    print("Filtered data is empty after removing rows with all-zero values.")
else:
    # Add cosine similarities to the filtered DataFrame
    df_filtered["Cosine Similarity"] = cosine_similarities[df_filtered.index.map(lambda x: file_names.index(x))]

    # Generate clustered heatmap
    def create_clustered_heatmap(data, index, columns, output_file):
        # Compute hierarchical clustering
        dist_matrix = pdist(data, metric="euclidean")
        linkage_matrix = linkage(dist_matrix, method="ward")
        ordered_indices = leaves_list(linkage_matrix)

        # Reorder data based on clustering
        clustered_data = data.iloc[ordered_indices, :]
        clustered_index = [index[i] for i in ordered_indices]

        # Create heatmap
        fig = px.imshow(
            clustered_data,
            labels=dict(x="Keywords", y="File", color="Value"),
            x=columns,
            y=clustered_index,
            title="Clustered Heatmap",
            aspect="auto",
        )
        fig.update_layout(height=len(index) * 20, width=800, xaxis=dict(tickangle=45))
        fig.write_html(output_file)

    # Generate the heatmap
    heatmap_output_file = os.path.join("{output_dir}", "clustered_heatmap.html")
    create_clustered_heatmap(
        df_filtered.drop(columns=["Cosine Similarity"]),
        df_filtered.index,
        keywords,
        heatmap_output_file
    )

    # K-Means clustering on cosine similarities
    num_clusters = 10 # was 3
    kmeans = KMeans(n_clusters=num_clusters, random_state=42)
    labels = kmeans.fit_predict(cosine_similarities[df_filtered.index.map(lambda x: file_names.index(x))].reshape(-1, 1))
    df_filtered["Cluster"] = labels

    # Save clustered scatter plot
    scatter_output_file = os.path.join("{output_dir}", "clustered_scatter.html")
    fig_clustered = px.scatter(
        df_filtered.reset_index(),
        x="Cluster",
        y="Cosine Similarity",
        title="K-Means Clustering of Cosine Similarities",
        color="Cluster",
        hover_name="index",
    )
    fig_clustered.write_html(scatter_output_file)

    # Generate interactive bar chart for keyword utilization
    keyword_totals = df.sum(axis=0)
    bar_chart_file = os.path.join("{output_dir}", "keyword_utilization_bar.html")
    fig_bar = px.bar(
        x=keywords,
        y=keyword_totals,
        title="Keyword Utilization Across All Files",
        labels=dict(x="Keywords", y="Total Count"),
    )
    fig_bar.update_layout(
        xaxis=dict(tickangle=45),
        height=500,
        width=1000
    )
    fig_bar.write_html(bar_chart_file)


# Generate heatmaps, adding cosine similarity
heatmaps = {{
    "log_transformed": df,
    "exponential_transformed": np.exp(df),
    "square_root_transformed": np.sqrt(df),
}}

for key, data in heatmaps.items():
    fig = px.imshow(data, 
                    labels=dict(x="Keywords", y="Files", color="Value"),
                    x=data.columns, 
                    y=file_names,
                    title=f"Heatmap: {{key.replace('_', ' ').capitalize()}}",
                    aspect="auto")

    html_file = f"{output_dir}/{{key}}_heatmap.html"

    # scaled view to be able to see all file names.
    fig.update_layout(height=len(heatmap_data)*15, width=800, xaxis=dict(tickangle=45))
    fig.write_html(html_file)

    # Automatically open all generated plots in the browser 
#    webbrowser.open(heatmap_output_file)
#    webbrowser.open(scatter_output_file)
#    webbrowser.open(bar_chart_file)
    
#    webbrowser.open(f"{output_dir}/log_transformed_heatmap.html")
#    webbrowser.open(f"{output_dir}/exponential_transformed_heatmap.html")
#    webbrowser.open(f"{output_dir}/square_root_transformed_heatmap.html")
"#,
        keywords = keywords,
        file_names = file_names,
        heatmap_data = heatmap_data,
        cosine_similarities = cosine_similarities,
        output_dir = output_dir
    )
} //end of generate_python_script

// add network script
// produce python script for another interactive network
pub fn generate_python_script_network_1(file_path: &str) -> String {
    format!(
        r#"
#!/usr/bin/env python3
# -*- coding: utf-8 -*-

import os
import pandas as pd
import networkx as nx
from pyvis.network import Network
import argparse


file_path = {file_path:?}

DATA_PATH = file_path 
# os.path.join(file_path,"/")
#print(f"\t NETWORK: Preparing network of results ...{file_path}")
#print(f"+++++++++++ data path = {{DATA_PATH}}")


DATA_FILE = "output.csv"
OUT_FILE = "network_graph"
# print(f"\t NETWORK: Preparing network of results ...0_out")

#######################################
# Work in steps to create the network.
#######################################

##################################################
## Step 1: Function to load CSV file using input()
##################################################


def load_csv_file(myFile):
    """load the csv file and return a dataframe."""
    if myFile == None: 
        print("\t No file to load! Exiting ...")
        exit()
    print(f"\t load_csv_file() : {{myFile}}")
    if myFile.lower().startswith("http"):  # If it's a URL, we'll try to fetch it
        try:
            df = pd.read_csv(myFile)
            print(f"\t * Data loaded from URL: {{myFile}}")
            return df
        except Exception as e:
            print(f"\t * Error loading data from URL: {{e}}")
            return None
    else:
        try:
            df = pd.read_csv(myFile)
            print(f"\t * Data loaded from file: {{myFile}}")
            return df
        except Exception as e:
            print(f"\t * Error loading file: {{e}}")
            return None

##############################################################
## Step 2: Function to create the graph based on the threshold
##############################################################


def create_graph_from_csv(df, threshold):
    # Initialize an empty graph
    G = nx.Graph()

    # Add nodes for each keyword and file, and then create edges based on the threshold
    for keyword in df.columns[1:-1]:  # Exclude 'Filename' and 'Cosine Similarity'
        G.add_node(keyword, type="keyword")  # Add the keyword node

    for _, row in df.iterrows():
        for keyword in df.columns[1:-1]:  # Exclude 'Filename' and 'Cosine Similarity'
            count = row[keyword]
            if count >= threshold:  # Only add edge if count is above threshold
                G.add_edge(row["Filename"], keyword, weight=count/3) # a third the width for the threshold

    return G


#####################################################
## Step 3: Function to visualize the graph with Pyvis
#####################################################


def visualize_graph(
    G,
    layout_algorithm="forceAtlas2",
    auto_render=True,
    keyword_color="lightblue",
    file_color="lightgreen",
    file_edge_colors=None,
    threshold=2,
):
    """A function to create the nodes and edges from the parameters."""
    net = Network(height="800px", width="90%", notebook=True)

    # Convert the NetworkX graph to Pyvis format
    net.from_nx(G)

    # Add specific attributes to each node in the pyvis graph
    for node in net.nodes:
        if node["id"] in G.nodes and "keyword" in node["id"]:  # For keyword nodes
            node["color"] = keyword_color  # Custom color for keyword nodes
            node["title"] = f"Keyword: {{node['id']}}"  # Tooltip for keywords
        else:  # For file nodes
            node["color"] = file_color  # Custom color for file nodes
            node["title"] = f"File: {{node['id']}}"  # Tooltip for filenames

    # Set the edge colors based on files
    for edge in net.edges:
        file_name = edge["from"]  # Get the file name from the edge
        if file_edge_colors and file_name in file_edge_colors:
            edge["color"] = file_edge_colors[
                file_name
            ]  # Set custom edge color for this file

    # Set the layout algorithm
    if layout_algorithm == "forceAtlas2":
        net.force_atlas_2based()
    elif layout_algorithm == "barnesHut":
        net.barnes_hut()


    # check that the data directory exists, create it if not.
    checkDataDir(DATA_PATH)

    # Save the network as an HTML file
    # create a file name with path
    saveFile = f"{{DATA_PATH}}/{{threshold}}_T_{{OUT_FILE}}.html"
    # saveFile = f"{{threshold}}_T_{{OUT_FILE}}.html"
    print("\t * File being saved as ",end ="")

    # add user control panel to the network
    # net.from_nx(nx.florentine_families_graph()) # sample data
    net.show_buttons(filter_=['physics'])
    net.toggle_physics(True)

    net.show(saveFile)

    # Optionally auto-render:: Let the user know about the creation of the network file. Cool right?!
    if auto_render:
        print(f"\t Interactive graph has been generated and saved as {{saveFile}}")
    else:
        print("\t Graph saved as {{saveFile}}. To view it, open the file in a browser.")


# end of visualize_graph()

def checkDataDir(dir_str):
    # function to determine whether a data output directory exists.
    # if the directory does not exist, then it is created

    try:
        os.makedirs(dir_str)
        # if DIR doesn't exist, create directory
        return 1

    except OSError:
        return 0

# end of checkDataDir()

#####################################################################
## Step 4: Main function to load CSV, create the graph, and render it
#####################################################################
def doesExist(fname: str) -> bool:
    """Checks for the existance of a file."""
    if os.path.exists(fname):
        print(f"\t * File: {{fname}} exists")
        return True
    else:
        print(f"\t * File: {{fname}} does not exist")
        return False


# end of doesExist()


def main():
    # Command line arguments setup using argparse
    parser = argparse.ArgumentParser(
        description="Generate a keyword-file network visualization."
    )

    # Input CSV file or URL
    parser.add_argument("--datafile", type=str, help="Path to the CSV file or URL")

    # Minimum threshold for keyword occurrence
    parser.add_argument(
        "--threshold",
        type=int,
        default=2,
        help="Minimum threshold for keyword occurrence (default: 2)",
    )

    # Layout algorithm: 'forceAtlas2' or 'barnesHut'
    parser.add_argument(
        "--layout",
        type=str,
        choices=["forceAtlas2", "barnesHut"],
        default="forceAtlas2",
        help="Layout algorithm (default: forceAtlas2)",
    )

    # Auto-render option: 'yes' or 'no'
    parser.add_argument(
        "--auto_render",
        type=str,
        choices=["yes", "no"],
        default="yes",
        help="Automatically render graph in browser? (default: yes)",
    )

    # Custom color for keyword nodes
    parser.add_argument(
        "--keyword_color",
        type=str,
        default="lightblue",
        help="Color for keyword nodes (default: lightblue)",
    )

    # Custom color for file nodes
    parser.add_argument(
        "--file_color",
        type=str,
        default="lightgreen",
        help="Color for file nodes (default: lightgreen)",
    )

    # Custom edge color for files
    parser.add_argument(
        "--file_edge_colors",
        type=str,
        help='JSON string to specify file-based edge colors (e.g., \'{{"PMC514601.txt": "red", "PMC516016.txt": "blue"}}\')',
    )

    # Parse command-line arguments
    args = parser.parse_args()

    # Load the CSV data
    df = load_csv_file(args.datafile)
    if df is None:
        return

    # Get the threshold and layout algorithm from the command-line arguments
    threshold = args.threshold
    layout_algorithm = args.layout
    auto_render = args.auto_render.lower() == "yes"
    keyword_color = args.keyword_color
    file_color = args.file_color

    # Parse file_edge_colors if provided
    file_edge_colors = {{}}
    if args.file_edge_colors:
        try:
            file_edge_colors = eval(
                args.file_edge_colors
            )  # Convert the JSON string into a dictionary
        except Exception as e:
            print(f"\t Error parsing file_edge_colors: {{e}}")
            return

    # Create the graph based on the threshold
    G = create_graph_from_csv(df, threshold)

    # Visualize the graph with the specified settings
    visualize_graph(
        G,
        layout_algorithm,
        auto_render,
        keyword_color,
        file_color,
        file_edge_colors,
        threshold,
    )


# end of main()

if __name__ == "__main__":
    print( "\t For help: programName -help")
    main()

# To execute:
# python3 script.py /path/to/data.csv --threshold 2 --layout forceAtlas2 --auto_render yes --keyword_color red --file_color blue --file_edge_colors '{{"PMC514601.txt": "red", "PMC516016.txt": "blue"}}'  Note, remove double curly brackets
# python3 script.py /path/to/data.csv --threshold 2 --layout forceAtlas2 --auto_render yes --keyword_color red --file_color blue --file_edge_colors '{{"PMC514601.txt": "red", "PMC516016.txt": "blue"}}'
# python3 myscript.py  output.csv --threshold 6 --layout forceAtlas2 --auto_render yes  --keyword_color purple --file_color lightblue
# notes: python3 network.py --datafile 0_out/obc.csv
        "#,
        file_path = { file_path }
    )
} // end of generate_python_script_network_1()

// produce python script for another interactive network
pub fn generate_python_script_network_2(file_path: &str) -> String {
    format!(
        r#"
#!/usr/bin/env python3
# -*- coding: utf-8 -*-

# we do not use this now, but it might be used later on.
file_path = {file_path:?}

DATA_PATH = file_path 
# os.path.join(file_path,"/")
#print(f"\t NETWORK: Preparing network of results ...{file_path}")
#print(f"+++++++++++ data path = {{DATA_PATH}}")



import pandas as pd
import networkx as nx
from pyvis.network import Network
import argparse
import json

# Function to load the CSV data
def load_csv_file(filepath):
    try:
        return pd.read_csv(filepath)
    except Exception as e:
        print(f"Error loading CSV file: {{e}}")
        return None


# Create a graph from the DataFrame with keyword occurrences filtered by threshold
def create_graph_from_csv(df, threshold):
    G = nx.Graph()

    # Add nodes for filenames
    for filename in df['Filename']:
        G.add_node(filename, type='file')

    # Add nodes for keywords and edges from filenames to keywords
    keywords = df.columns[1:-1]  # All columns except for 'Filename' and 'Cosine Similarity'
    for keyword in keywords:
        G.add_node(keyword, type='keyword')  # Add keyword nodes

    # Add edges based on keyword occurrences and threshold
    for index, row in df.iterrows():
        filename = row['Filename']
        for keyword in keywords:
            count = row[keyword]
            if count >= threshold:  # Only add edges for keywords that meet the threshold
                G.add_edge(filename, keyword, weight=count)

    return G


# Generate an interactive network visualization using pyvis
def visualize_interactive_graph(G, output_file='network.html', keyword_color="lightblue", file_color="lightgreen"):
    # Create a PyVis network object
    net = Network(notebook=True, height="800px", width="90%")
    
    # Add nodes and edges from NetworkX graph to PyVis
    net.from_nx(G)

    # Set the physics layout to true for interactive node movement
    net.force_atlas_2based()

    # Optionally, adjust the appearance of nodes, edges, etc.
    net.set_options("""
    var options = {{
        "nodes": {{
            "shape": "dot",
            "size": 15,
            "color": {{
                "highlight": {{
                    "background": "orange",
                    "border": "red"
                }}
                    }}
                            }},
        "edges": {{
            "smooth":   {{
                "type": "continuous"
                }}
                        }}
                            }}
    """)

    # Customize node colors based on type (keyword/file)
    for node in net.nodes:
        if node['label'] in G.nodes:
            if G.nodes[node['label']]['type'] == 'keyword':
                node['color'] = keyword_color  # Set color for keyword nodes
            elif G.nodes[node['label']]['type'] == 'file':
                node['color'] = file_color  # Set color for file nodes

    # Save the visualization as an HTML file
    print("\t * File being saved as ",end ="")


#    # add user control panel to the network
# seems to have a bug when this code is in use... 
#    # net.from_nx(nx.florentine_families_graph()) # sample data
#    net.show_buttons(filter_=['physics'])
#    net.toggle_physics(True)
    net.show(output_file)
    print(f"\t Interactive network visualization saved as {{output_file}}")


# Generate a report on the node degrees with detailed connection information
def generate_node_degree_report(G, df, output_file=None, detailed_report=False):
    # Calculate the degrees for keyword nodes and file nodes
    print("\t Generating report . . .")
    keyword_degrees = {{}}
    file_degrees = {{}}

    # Store detailed connection info
    detailed_info = []

    for node in G.nodes():
        if G.nodes[node]['type'] == 'keyword':
            keyword_degrees[node] = G.degree(node)
            if detailed_report:
                # Report detailed information about which files are connected to this keyword
                connected_files = []
                for file in G.neighbors(node):
                    count = df[df['Filename'] == file][node].values[0]
                    connected_files.append(f"{{file}} (count: {{count}})")
                detailed_info.append(f"Keyword '{{node}}' connected to files: {{', '.join(connected_files)}}")

        elif G.nodes[node]['type'] == 'file':
            file_degrees[node] = G.degree(node)
            if detailed_report:
                # Report detailed information about which keywords are connected to this file
                connected_keywords = []
                for keyword in G.neighbors(node):
                    count = df[df['Filename'] == node][keyword].values[0]
                    connected_keywords.append(f"{{keyword}} (count: {{count}})")
                detailed_info.append(f"File '{{node}}' connected to keywords: {{', '.join(connected_keywords)}}")

    # Prepare the report
    report = []

    report.append("=== Keyword Node Degrees ===")
    for keyword, degree in keyword_degrees.items():
        report.append(f"{{keyword}}: {{degree}} connections")

    report.append("\n=== File Node Degrees ===")
    for file, degree in file_degrees.items():
        report.append(f"{{file}}: {{degree}} connections")

    # Include detailed connections if requested
    if detailed_report:
        report.append("\n=== Detailed Connections ===")
        report.extend(detailed_info)

    # Optionally save the report to a file
    if output_file:
        with open(output_file, 'w') as f:
            f.write("\n".join(report))
        print(f"Node degree report saved to {{output_file}}")
    else:
        print("\n".join(report))


# Command-line interface setup
def main():
    # Command line arguments setup using argparse
    parser = argparse.ArgumentParser(
        description="Generate a keyword-file network visualization."
    )

    # Input CSV file or URL
    parser.add_argument("--datafile", type=str, required=True, help="Path to the CSV file or URL")

    # Minimum threshold for keyword occurrence
    parser.add_argument(
        "--threshold",
        type=int,
        default=2,
        help="Minimum threshold for keyword occurrence (default: 2)",
    )

    # Layout algorithm: 'forceAtlas2' or 'barnesHut' (only spring_layout supported in this version)
    parser.add_argument(
        "--layout",
        type=str,
        choices=["forceAtlas2", "barnesHut"],
        default="spring_layout",
        help="Layout algorithm (default: spring_layout)",
    )

    # Auto-render option: 'yes' or 'no'
    parser.add_argument(
        "--auto_render",
        type=str,
        choices=["yes", "no"],
        default="yes",
        help="Automatically render graph in browser? (default: yes)",
    )

    # Custom color for keyword nodes
    parser.add_argument(
        "--keyword_color",
        type=str,
        default="lightblue",
        help="Color for keyword nodes (default: lightblue)",
    )

    # Custom color for file nodes
    parser.add_argument(
        "--file_color",
        type=str,
        default="lightgreen",
        help="Color for file nodes (default: lightgreen)",
    )

    # Custom edge color for files
    parser.add_argument(
        "--file_edge_colors",
        type=str,
        help='JSON string to specify file-based edge colors (e.g., \'{{"PMC514601.txt": "red", "PMC516016.txt": "blue"}}\')',
    )

    # Output file for degree report
    parser.add_argument(
        "--degree_report",
        type=str,
        help="Path to save the node degree report (default: None)"
    )

    # Option for detailed report (include connections)
    parser.add_argument(
        "--detailed_report",
        action='store_true',
        help="Include detailed connections in the node degree report (default: False)"
    )

    # Output file for the interactive network visualization
    parser.add_argument(
        "--output_file",
        type=str,
        default="network.html",
        help="Name of the output HTML file (default: 'network.html')"
    )

    # Parse command-line arguments
    args = parser.parse_args()

    dataFile = args.datafile
    if not dataFile:
        print("No data file specified. Exiting.")
        return

    # Load the CSV DATA_FILE
    df = load_csv_file(dataFile)
    if df is None:
        print("Failed to load the data file. Exiting.")
        return

    # Get parameters from arguments
    threshold = args.threshold
    layout_algorithm = args.layout
    auto_render = args.auto_render.lower() == "yes"
    keyword_color = args.keyword_color
    file_color = args.file_color

    # Parse file_edge_colors if provided
    file_edge_colors = {{}}
    if args.file_edge_colors:
        try:
            file_edge_colors = json.loads(args.file_edge_colors)
        except Exception as e:
            print(f"Error parsing file_edge_colors: {{e}}")
            return

    # Create the graph based on the threshold
    G = create_graph_from_csv(df, threshold)

    # Generate and optionally save the degree report
    if args.degree_report:
        generate_node_degree_report(G, df, output_file=args.degree_report, detailed_report=args.detailed_report)

    # add extension to the output file
    output_file=args.output_file + ".html"

    # Visualize the graph interactively
    visualize_interactive_graph(G, output_file, keyword_color=keyword_color, file_color=file_color)

    # Former code, maybe we can use it later when we figure out the extension checking...
    #visualize_interactive_graph(G, output_file=args.output_file, #keyword_color=keyword_color, file_color=file_color)


if __name__ == "__main__":
    main()
# run: python3 myscript.py --datafile output.csv --threshold 2 --output_file network.html --keyword_color lightblue --file_color lightgreen
# with report
# run: python3 myscript.py --datafile output.csv --threshold 10 --output_file network.html --keyword_color lightblue --file_color lightgreen --layout forceAtlas2 --degree_report degree_report.txt --detailed_report

"#,
        file_path = { file_path }
    )
} // end of generate_python_script_network_2()

// produce python script for another interactive network
pub fn generate_python_script_complete_network(file_path: &str) -> String {
    format!(
        r#"
#!/usr/bin/env python3
# -*- coding: utf-8 -*-

# we do not use this now, but it might be used later on.
file_path = {file_path:?}

OUTPUT_DIR = file_path 
# os.path.join(file_path,"/")
#print(f"\t NETWORK: Preparing network of results ...{file_path}")
#print(f"+++++++++++ data path = {{DATA_PATH}}")
####
#!/usr/bin/env python3
# -*- coding: utf-8 -*-

import pandas as pd
import networkx as nx
from pyvis.network import Network
import os

# OUTPUT_DIR = "0_out/"

def checkDataDir(dir_str):
    """ function to determine whether a data output directory exists. 
    if the directory does not exist, then it is created """

    try:
        os.makedirs(dir_str)
        # if DIR doesn't exist, create directory
        return 1

    except OSError:
        return 0

# end of checkDataDir()

def load_data(filename):
    """ Load the CSV data into a pandas DataFrame """
    return pd.read_csv(filename)

def create_graph_from_csv(df, threshold):
    """
    Create a network graph where:
    - Nodes represent both keywords and files.
    - Edges connect a keyword to a file if the keyword is found in the file with occurrences above the threshold.
    """
    G = nx.Graph()

    # Convert the keyword columns to numeric values (if not already)
    df.iloc[:, 1:] = df.iloc[:, 1:].apply(pd.to_numeric, errors='coerce')

    # Add file nodes to the graph
    for _, row in df.iterrows():
        file_name = row['Filename']
        G.add_node(file_name, type='file')

    # Add keyword nodes and create edges based on the threshold
    for col in df.columns[1:]:
        if col != 'Filename':  # Skip the 'Filename' column
            G.add_node(col, type='keyword')
            for _, row in df.iterrows():
                # Add an edge if the keyword appears in the file and exceeds the threshold
                if pd.notna(row[col]) and row[col] > threshold:
                    G.add_edge(row['Filename'], col, weight=row[col])

    return G

def visualize_graph(G, output_filename, layout='spring', node_color='lightblue', edge_color='gray'):
    """ Visualize the graph using pyvis """
    # print(f" visualize_graph() parameter: {{G}},\n output_filename: {{output_filename}},\n{{layout}},\n{{node_color}},\n{{edge_color}}")

    net = Network(height='800px', width='90%', notebook=True)

    # Add nodes and edges to the network
    for node, data in G.nodes(data=True):
        if data['type'] == 'keyword':
            net.add_node(node, color='red', size=15)
        else:
            net.add_node(node, color=node_color, size=10)

    for u, v, data in G.edges(data=True):
        net.add_edge(u, v, width=data['weight'], color=edge_color)

    # TODO
    # Set the layout (adjustable by user)
    # net.force_atlas_2based() if layout == 'force' else net.spring_layout()
    net.force_atlas_2based()

    # add user control panel to the network
    # net.from_nx(nx.florentine_families_graph()) # sample data
    net.show_buttons(filter_=['physics'])
    net.toggle_physics(True)

    checkDataDir(OUTPUT_DIR) # verify that output directory exists, create if not

# Example of using os.path.join
# path = os.path.join("/home", "user", "documents", "/etc", "config.txt")
# print(path)

    fname = os.path.join(OUTPUT_DIR, output_filename)
    print(f"fname: {{fname}}")

    # Save the graph to an HTML file
    # net.show(output_filename) # raises an error due... !
    # net.show(output_filename, notebook=False) #default, no path details
    net.show(fname, notebook=False)

def parse_arguments():
    """ Parse command-line arguments """
    import argparse
    parser = argparse.ArgumentParser(description="Generate an interactive network from a CSV file")
    parser.add_argument('-f', '--file', required=True, help="CSV file with keyword counts")
    parser.add_argument('-t', '--threshold', type=int, default=0, help="Threshold for keyword occurrence")
    parser.add_argument('-l', '--layout', choices=['spring', 'force'], default='spring', help="Layout for the network")
    parser.add_argument('-nr', '--no_render', action='store_true', help="Do not automatically render the HTML file")
    parser.add_argument('-nc', '--node_color', default='lightblue', help="Color for file nodes")
    parser.add_argument('-ec', '--edge_color', default='gray', help="Color for edges")

    return parser.parse_args()

def main():
    args = parse_arguments()

    # Load the data from CSV
    df = load_data(args.file)
    # print(f"df : {{df}}")

    # Create the graph from the data
    G = create_graph_from_csv(df, args.threshold)
    # print(f"G is : {{G}}")

    # Visualize the graph and output it as an HTML file
    #output_filename = 'interactive_network.html'
    output_filename = 'complete_network.html'
    visualize_graph(G, output_filename, layout=args.layout, node_color=args.node_color, edge_color=args.edge_color)

    if not args.no_render:
        print(f"\t Graph rendered to {{output_filename}}. Open the file in your browser.")
    else:
        print(f"Graph saved as {{output_filename}}.")

if __name__ == "__main__":
    main()

# run: python3 programName.py -f output.csv -t 6 -nc blue -ec green 

####
"#,
        file_path = { file_path }
    )
} // end of generate_python_script_complete_network()
