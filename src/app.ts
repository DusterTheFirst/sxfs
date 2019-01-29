/*!
 * Copyright (C) 2019  Zachary Kohnen
 */

import bodyParser from "body-parser";
import compression from "compression";
import express, { NextFunction, Request, Response } from "express";
import basicAuth from "express-basic-auth";
import handlebars from "express-handlebars";
import filesize from "filesize";
import { existsSync, mkdirSync, readdirSync, statSync, unlinkSync, writeFileSync } from "fs";
import helmet from "helmet";
import { getExtension } from "mime";
import moment from "moment";
import multer from "multer";
import { join } from "path";
import { ulid } from "ulid";
import mimemap from "./mimemap.json";
import users from "./users.json";

// Create dir pointers
const UPLOAD_DIR = join(__dirname, "..", "uploads");
const VIEWS_DIR = join(__dirname, "views");
const STATIC_DIR = join(__dirname, "static");

// Make sure the upload directory exists
if (!existsSync(UPLOAD_DIR)) {
    mkdirSync(UPLOAD_DIR);
}

// Create an express app
let app = express();

// Create a file uploader
let upload = multer();

// Create a basic authentication service
const AUTH = basicAuth({
    challenge: true,
    unauthorizedResponse: "This is a file host server, you should not navigate here directly",
    users
});

// Setup app middleware
app.use(compression());
app.use(helmet({ noCache: true }));

// Setup handlebars
app.engine(".hbs", handlebars({ layoutsDir: VIEWS_DIR, extname: ".hbs" }));
app.set("view engine", ".hbs");
app.set("views", VIEWS_DIR);

interface IMainQuery {
    sort?: "size";
}
// Main page
app.get("/", AUTH, (req, res) => res.render("files", {
    // Get the files
    files: readdirSync(UPLOAD_DIR)
        // Get file info
        .map(x => {
            let stat = statSync(join(UPLOAD_DIR, x));

            return {
                name: x,
                size: stat.size,
                uploaded: stat.birthtime
            };
        })
        // Sort the files
        .sort((a, b) => {
            if ((req.query as IMainQuery).sort === "size") {
                return b.size - a.size;
            } else {
                return b.name.localeCompare(a.name);
            }
        })
        // Clean the info for display
        .map(x => ({
            name: x.name,
            size: filesize(x.size, { standard: "jedec" }),
            uploaded: moment(x.uploaded).fromNow()
        })),
    sort: (req.query as IMainQuery).sort
}));

// Upload form
app.get("/upload", AUTH, (_req, res) => res.render("upload"));

// Static files
app.use("/static", express.static(STATIC_DIR));

// Upload endpoint
interface IUploadBody {
    user?: string;
}
app.post("/upload", bodyParser.urlencoded({ extended: true }), upload.single("img_file"), AUTH, (req, res) => {
    // Genetate a ULID for the file
    let uuid = ulid();

    // Replace unknown/out of date mime types with well known ones
    let mappedMimeType = Object.keys(mimemap).includes(req.file.mimetype) ? (mimemap as { [x: string]: string })[req.file.mimetype] : req.file.mimetype;

    // Generate a filename for the image
    let publicfilename = `${uuid}.${getExtension(mappedMimeType)}`;

    // Save the file
    writeFileSync(join(UPLOAD_DIR, publicfilename), req.file.buffer);

    // Check if user submitted
    if ((req.body as IUploadBody).user !== undefined) {
        // Redirect to the new file
        res.redirect(publicfilename);
    } else {
        // Send the new filename
        res.send(publicfilename);
    }
});

// Delete all files
app.get("/delete/all", AUTH, (_req, res) => {
    readdirSync(UPLOAD_DIR).forEach(x => unlinkSync(join(UPLOAD_DIR, x)));
    res.redirect("/");
});

// Delete a specific file
app.get("/delete/:file", AUTH, (req, res) => {
    let { file } = req.params as { file: string };

    // Make sure the file exists
    if (existsSync(join(UPLOAD_DIR, file))) {
        unlinkSync(join(UPLOAD_DIR, file));
        res.redirect("/");
    } else {
        res.render("404", { delete: true, file });
    }
});

// Static uploads
app.use("/", express.static(UPLOAD_DIR, { lastModified: true }));

// Fallback 404
app.use("*", (req, res) => res.render("404", { url: req.originalUrl }));

// Fallback 500
app.use((err: Error, _req: Request, res: Response, _next: NextFunction) => {
    res.render("500");
    console.error(err);
});

app.listen(4299, () => {
    console.log("ShareX server started on port 4299");
});