const fileInput = /** @type {HTMLInputElement} */ (
	document.getElementById("file-input")
);

fileInput.addEventListener("change", uploadFile);

async function uploadFile() {
	const file = /** @type {FileList} */ (fileInput.files)[0];
	const text = await file.text();
	const csv = parseCSV(text);
	const models = [...new Set(csv.map((entry) => entry.idx))];

	const tableElement = /** @type {HTMLTableElement} */ (
		document.getElementById("table")
	);

	const traces = [];

	for (const idx of models) {
		const data = csv.filter((entry) => entry.idx === idx);

		const vendor = data[0].vendor;
		const model = data[0].model;
		traces.push({
			x: data.map((entry) => entry.time),
			y: data.map((entry) => entry.percentage),
			text: data.map((entry) => {
				let result = `State: ${entry.state}<br>`;
				if (entry.state === "charging") {
					result += `Time to full: ${entry.time_to_full}`;
				} else {
					result += `Time to empty: ${entry.time_to_empty}`;
				}
				return result;
			}),
			name: `${idx}: ${vendor} ${model}`,
			type: "scatter",
			hovertemplate:
				"<b>%{yaxis.title.text}: %{y:.0%}</b><br>" +
				"<b>%{text}</b><br><br>",
		});
	}
	const layout = {
		title: "Battery percentage over time",
		xaxis: {
			title: "Time",
		},
		yaxis: {
			title: "Percentage",
		},
	};
	// @ts-ignore
	Plotly.newPlot("chart", traces, layout);
}

/**
 * @typedef {{time: string, idx: number, vendor: string, model: string, state: string, percentage: number, time_to_full: string, time_to_empty: string}[]} CSV
 */

/**
 * Parses a CSV object
 * @param {string} data
 * @returns {CSV}
 */
function parseCSV(data) {
	const lines = data
		.replace("\r", "")
		.split("\n")
		.filter((line) => line.trim() !== "");
	/** @type {CSV} */
	const result = [];

	for (let i = 1; i < lines.length; i++) {
		const values = lines[i].split(",");
		const obj = {};
		obj.time = values[0];
		obj.idx = parseInt(values[1]);
		obj.vendor = values[2];
		obj.model = values[3];
		obj.state = values[4];
		obj.percentage = parseFloat(values[5]);
		obj.time_to_full = values[6];
		obj.time_to_empty = values[7];
		result.push(obj);
	}

	return result;
}
