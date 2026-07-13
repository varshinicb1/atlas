---
kind: Package
id: package:wembi
name: Wembi Digital Twin Platform
version: "1.0"
purpose: "Document UI/UX and architectural patterns from wembi.ai — a Digital Twin platform for IoT/industrial device management. Italian B2B SaaS with predictive maintenance, real-time monitoring, and remote device control."
problem_solved: "Reference for designing B2B IoT/IIoT platforms — digital twin visualization, real-time device dashboards, predictive maintenance interfaces, and multi-language (Italian/English) enterprise SaaS patterns."
install: ""
dependencies:
  - package:react
  - package:nextjs
  - package:tailwind
concepts:
  - name: Digital Twin Visualization
    id: concept:wembi/digital-twin-viz
    description: "Real-time virtual replica of physical IoT devices rendered in a browser dashboard; mirrors device state, telemetry, and operational status."
  - name: Real-time Device Monitoring
    id: concept:wembi/realtime-monitoring
    description: "Live data streams from field devices normalized into a single-pane-of-glass interface with charts, gauges, and status indicators."
  - name: Predictive Maintenance
    id: concept:wembi/predictive-maintenance
    description: "ML-driven forecasting that predicts device failures before they occur; surfaces recommended actions and risk scores in the dashboard."
  - name: Remote Device Control
    id: concept:wembi/remote-control
    description: "Bidirectional command dispatch from dashboard to physical devices — start, stop, configure, and restart machinery remotely with audit trails."
  - name: OT-IT Architecture Integration
    id: concept:wembi/ot-it-integration
    description: "Open-source distributed architecture bridging Operational Technology (field devices, sensors, PLCs) with Information Technology (cloud dashboards, APIs, analytics)."
  - name: Multi-language Enterprise SaaS
    id: concept:wembi/multilang-saas
    description: "Full Italian and English platform localization for B2B IoT SaaS — terminology consistency across device labels, alerts, and compliance docs."
  - name: IoT Data Acquisition Pipeline
    id: concept:wembi/data-pipeline
    description: "End-to-end ingestion flow from field device protocols to normalized dashboard-ready data — collection, normalization, analysis, and visualization."
  - name: Industrial Dashboard Design
    id: concept:wembi/industrial-dashboard
    description: "Dark-themed, high-information-density dashboards optimized for industrial operators — real-time gauges, alert panels, and device topology maps."
  - name: Pay-per-Use Billing Model
    id: concept:wembi/pay-per-use
    description: "Consumption-based pricing for IoT platform access — no license fees, no vendor lock-in, metered by device connections and data volume."
  - name: Edge-to-Cloud Simulation
    id: concept:wembi/edge-simulation
    description: "Simulate device behavior under hypothetical scenarios (production scaling, energy optimization) before applying changes to physical assets."
apis:
  - name: Device Data API
    id: api:wembi/device-data
    signature: "GET /api/v1/devices/:id/telemetry?from=&to=&resolution="
    returns: "Time-series telemetry data array with timestamps, metrics, and device status."
    description: "Retrieves normalized telemetry data from field devices with configurable time range and resolution for dashboard charting."
  - name: Real-time Telemetry Stream
    id: api:wembi/telemetry-stream
    signature: "WebSocket /ws/devices/:id/stream"
    returns: "Live telemetry frames as JSON — metric values, status changes, and alert triggers."
    description: "WebSocket connection for pushing live device telemetry to dashboard widgets without polling overhead."
  - name: Predictive Alert System
    id: api:wembi/predictive-alert
    signature: "GET /api/v1/alerts/predictive?device_id=&severity="
    returns: "Array of predicted failure events with probability scores, time windows, and recommended actions."
    description: "Returns ML-generated failure predictions ranked by probability so operators can prioritize maintenance."
  - name: Remote Command Dispatch
    id: api:wembi/remote-command
    signature: "POST /api/v1/devices/:id/command"
    returns: "Command acknowledgement with execution status, timestamp, and device-side result."
    description: "Sends operational commands (start, stop, configure, restart) to physical devices with full audit logging."
  - name: Analytics Dashboard
    id: api:wembi/analytics-dashboard
    signature: "GET /api/v1/analytics/summary?period=&device_group="
    returns: "Aggregated KPIs — uptime, efficiency, energy consumption, anomaly counts, and trend charts."
    description: "Provides pre-computed analytics summaries for dashboard widgets showing fleet-wide operational metrics."
  - name: Device Registry
    id: api:wembi/device-registry
    signature: "GET /api/v1/devices?page=&per_page=&status=&type="
    returns: "Paginated device list with metadata, connection status, firmware version, and location."
    description: "CRUD API for managing the device inventory — registration, decommissioning, firmware updates, and grouping."
  - name: User Permission Model
    id: api:wembi/user-permissions
    signature: "GET /api/v1/users/:id/permissions"
    returns: "Permission set object — device groups, dashboard access, command authorization, and API rate limits."
    description: "Role-based access control mapping users to device groups with granular read/control/admin tiers."
  - name: Integration Webhook
    id: api:wembi/integration-webhook
    signature: "POST /api/v1/webhooks/:id/trigger"
    returns: "Webhook delivery status with attempt count, last response, and next retry schedule."
    description: "Outbound webhooks for integrating IoT events (alerts, status changes, predictive warnings) into external systems."
  - name: Device Topology API
    id: api:wembi/device-topology
    signature: "GET /api/v1/devices/topology?site_id="
    returns: "Hierarchical device tree with parent-child relationships, connection types, and data flow paths."
    description: "Returns the OT network topology showing how field devices, gateways, and controllers interconnect."
  - name: Simulation Engine
    id: api:wembi/simulation
    signature: "POST /api/v1/simulations"
    returns: "Simulation job ID with estimated completion time and input parameter summary."
    description: "Kicks off a what-if simulation on a digital twin — operators provide scenario parameters and get projected outcomes."
examples:
  - id: example:wembi/dashboard-layout
    language: tsx
    description: "Three-panel industrial dashboard — device topology tree on the left, real-time gauges center, alert feed on the right."
  - id: example:wembi/digital-twin-card
    language: tsx
    description: "Digital Twin device card showing live telemetry, connection status, and quick-action command buttons."
  - id: example:wembi/predictive-alert-timeline
    language: tsx
    description: "Horizontal timeline of predicted failure events with probability bars and recommended maintenance windows."
  - id: example:wembi/command-dialog
    language: tsx
    description: "Remote command confirmation dialog with parameter inputs, execution preview, and audit trail display."
  - id: example:wembi/multilang-toggle
    language: tsx
    description: "Language switcher component that re-fetches all dashboard labels and device metadata in the target locale."
failures:
  - id: failure:wembi/stale-telemetry
    symptom: "Dashboard charts show flat lines or stale data while devices are actively transmitting."
    cause: "Telemetry polling interval is too long or WebSocket reconnection logic is missing on disconnect."
    fix: "Use WebSocket for live streams with automatic reconnection; set dashboard 'last updated' timestamps and stale-data warnings."
  - id: failure:wembi/command-no-confirmation
    symptom: "Operators accidentally trigger destructive commands (restart, shutdown) on production devices."
    cause: "Remote command dispatch lacks confirmation step, permission check, or impact preview before execution."
    fix: "Add multi-step confirmation dialogs with execution preview, require elevated permissions for destructive actions, and log all commands."
  - id: failure:wembi/iot-data-overload
    symptom: "Dashboard crashes or becomes unresponsive when hundreds of devices stream telemetry simultaneously."
    cause: "No client-side data aggregation, virtualization, or rendering limits on chart series."
    fix: "Downsample telemetry at the API level, virtualize chart data, and limit visible device count per dashboard view."
  - id: failure:wembi/ot-it-latency
    symptom: "Commands take seconds to reach field devices; dashboard status lags behind physical reality."
    cause: "Cloud round-trip latency between dashboard UI and OT network without edge processing or local caching."
    fix: "Deploy edge gateways for local command dispatch and status caching; use optimistic UI updates with deferred confirmation."
  - id: failure:wembi/localization-gaps
    symptom: "Half the dashboard labels are in English while device metadata stays in Italian; inconsistent terminology."
    cause: "Localization is applied only to UI chrome, not to dynamic device names, alert messages, or compliance labels."
    fix: "Store translated device metadata in the registry; provide locale-aware alert templates and enforce translation coverage thresholds."
extends: []
uses:
  - concept:wembi/digital-twin-viz
  - concept:wembi/realtime-monitoring
  - concept:wembi/predictive-maintenance
part_of: concept:domain/iot-platforms
depends_on:
  - package:react
  - package:nextjs
  - package:tailwind
solves:
  - problem:iot-dashboard-design
alternatives:
  - package:aws-iot-twinmaker
  - package:azure-digital-twins
---
# Wembi Digital Twin Platform

Wembi is an Italian B2B SaaS platform that creates instant digital twins of any IoT device, machine, or digital apparatus. It combines real-time monitoring, predictive maintenance, and remote device control into a single dashboard with full Italian and English localization.

The platform follows a three-layer architecture: Data acquisition and normalization from field devices, Device management and remote control, and Application-layer rules for automation, simulation, and AI integration. The OT-IT architecture is fully open-source with a pay-per-use billing model — no license fees, no vendor lock-in.

The dashboard is organized around three views: a device topology tree on the left for navigation, a center pane with real-time gauges and telemetry charts, and an alert feed on the right showing predictive maintenance warnings and system events. Each device card shows live telemetry, connection status, and quick-action command buttons. Commands go through a multi-step confirmation dialog with execution preview and full audit trails.

Predictive maintenance uses ML-driven forecasting displayed as a horizontal timeline with probability bars and recommended maintenance windows. Operators can run what-if simulations on digital twins before applying changes to physical assets — testing production scaling scenarios and energy optimization strategies without risk.

Telemetry flows through WebSocket connections for live streaming with automatic reconnection. The Device Registry API provides full CRUD for device inventory, firmware version tracking, and site grouping. The Integration Webhook system pushes IoT events (alerts, status changes, predictive warnings) into external systems for enterprise workflows.
