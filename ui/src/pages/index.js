import Image from "next/image";
import { TrashSimple, CaretDown, Plus, PencilSimple } from "phosphor-react";
import React, { Fragment, useEffect, useRef, useState } from "react";
import axios from "axios";
import { SERVER_ADDRESS } from "../types/constants";
import { Dialog, Transition } from "@headlessui/react";
import ReactPaginate from "react-paginate";

const CONTAINS = "chứa";
const NOT_CONTAINS = "không chứa";
const TITLE = "tiêu đề";
const CONTENT = "nội dung";
const suggestions = [
  `${TITLE} ${CONTAINS}`,
  `${TITLE} ${NOT_CONTAINS}`,
  `${CONTENT} ${CONTAINS}`,
  `${CONTENT} ${NOT_CONTAINS}`,
];

export default function ShowcaseHN() {
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState("");
  const [success, setSuccess] = useState(false);
  const [submitted, setSubmitted] = useState(false);
  const [jobResults, setJobResults] = useState([]);
  const [jobCount, setJobCount] = useState(0);
  const [propertyMode, setPropertyMode] = useState(null);
  const [fields, setFields] = useState([
    {
      name: `${CONTENT} ${CONTAINS}`,
      description: "",
      criteria: "đội tuyển",
      type: "",
    },
    // {
    //   name: `${CONTENT} ${NOT_CONTAINS}`,
    //   description: "",
    //   criteria: "Thái Lan",
    //   type: "",
    // },
    {
      name: `${TITLE} ${CONTAINS}`,
      description: "",
      criteria: "Việt Nam",
      type: "",
    },
    // {
    //   name: `${TITLE} ${NOT_CONTAINS}`,
    //   description: "",
    //   criteria: "Park",
    //   type: "",
    // },
  ]);
  const parseFieldsToQuery = () => {
    let query = "";
    fields.forEach((f) => {
      if (query.length > 0) {
        query += f.type === "or" ? " OR " : " AND ";
      }
      if (f.name === `${TITLE} ${CONTAINS}`) {
        query += `(title:"${f.criteria}")`;
      } else if (f.name === `${TITLE} ${NOT_CONTAINS}`) {
        query += `NOT (title:"${f.criteria}")`;
      } else if (f.name === `${CONTENT} ${CONTAINS}`) {
        query += `(content:"${f.criteria}")`;
      } else if (f.name === `${CONTENT} ${NOT_CONTAINS}`) {
        query += `NOT (content:"${f.criteria}")`;
      }
    });
    console.log(query);
    return query;
  };

  function Items({ currentItems }) {
    return (
      <>
        {currentItems &&
          currentItems.map((job) => (
            <li key={job.url} className="flex flex-wrap gap-2">
              <div className="flex flex-wrap gap-2 p-2 rounded place-items-center bg-stone-100">
                <div className="flex flex-col flex-wrap w-full gap-2 ">
                  <div className="flex-1 font-bold">{job.title}</div>
                  <div className="font-medium shrink-0">{job.summary}</div>
                  <a
                    href={job.url}
                    target="_blank"
                    className="flex-1 font-medium text-blue-600"
                  >
                    Xem thêm
                  </a>
                </div>
              </div>
            </li>
          ))}
      </>
    );
  }
  function PaginatedItems({ itemsPerPage }) {
    // Here we use item offsets; we could also use page offsets
    // following the API or data you're working with.
    const [itemOffset, setItemOffset] = useState(0);

    // Simulate fetching items from another resources.
    // (This could be items from props; or items loaded in a local state
    // from an API endpoint with useEffect and useState)
    const endOffset = itemOffset + itemsPerPage;
    console.log(`Loading items from ${itemOffset} to ${endOffset}`);
    const currentItems = jobResults.slice(itemOffset, endOffset);
    const pageCount = Math.ceil(jobResults.length / itemsPerPage);

    // Invoke when user click to request another page.
    const handlePageClick = (event) => {
      const newOffset = (event.selected * itemsPerPage) % jobResults.length;
      console.log(
        `User requested page number ${event.selected}, which is offset ${newOffset}`
      );
      setItemOffset(newOffset);
    };

    return (
      <>
        <ReactPaginate
          breakLabel="..."
          nextLabel="next >"
          onPageChange={handlePageClick}
          pageRangeDisplayed={3}
          pageCount={pageCount}
          previousLabel="< previous"
          renderOnZeroPageCount={null}
          containerClassName="flex justify-center my-4 w-full gap-2 text-sm font-medium text-slate-800 "
          pageClassName="mx-1 px-2 py-1 border rounded hover:bg-slate-800 hover:text-white"
          activeClassName="bg-slate-800 text-white"
          pageLinkClassName="hover:bg-slate-800 hover:text-white"
          disabledClassName="opacity-50 cursor-not-allowed"
          previousClassName="mx-1 px-2 py-1 border rounded"
          nextClassName="mx-1 px-2 py-1 border rounded"
          breakClassName="mx-1 px-2 py-1 border rounded"
        />
        <Items currentItems={currentItems} />
      </>
    );
  }
  const [selectedProperty, setSelectedProperty] = useState(null);
  const [formData, setFormData] = useState({
    name: "",
    description: "",
    type: "",
    criteria: "",
  });

  const handleEditClick = (property, index) => {
    setFormData({
      name: property.name,
      description: property.description,
      type: property.type,
      criteria: property.criteria,
    }); // Populate this with real data
    setSelectedProperty(index);
    setPropertyMode("edit");
  };

  const handleDeleteClick = (event, index) => {
    event.preventDefault();
    const newFields = fields.filter((field, i) => i !== index);
    setFields(newFields);
  };

  const handleSaveChangesClick = (index) => {
    let newFields = [...fields];
    if (propertyMode === "edit") {
      newFields[index] = formData;
    } else if (propertyMode === "add") {
      newFields.push(formData);
    }
    setFields(newFields);
    setPropertyMode(null);
    setFormData({ name: "", description: "", criteria: "", type: "" });
  };

  const handleCancelClick = () => {
    setPropertyMode(null);
  };

  useEffect(() => {
    load();
  }, []);

  const handleSubmit = async (e) => {
    e.preventDefault();
    setSubmitted(true);
    await search();
  };

  const search = async () => {
    setLoading(true);
    setSuccess(false);
    try {
      const res = await axios.post(`${SERVER_ADDRESS}/api/articles/query`, {
        query: parseFieldsToQuery(),
      });
      const data = res.data?.data;
      const result = { data: [] };
      data.forEach((item) => {
        result.data.push(JSON.parse(item));
      });
      setJobResults(result.data);
      setJobCount(res.data.article_count);
    } catch (error) {
      console.error(error);
    } finally {
      setLoading(false);
    }
  };

  const load = async () => {
    setLoading(true);
    setSuccess(false);
    try {
      const res = await axios.get(`${SERVER_ADDRESS}/load`);
      const data = res.data;
      if (data.results?.length > 0) {
        setJobResults(
          data.results.sort((a, b) => b.relevanceScore - a.relevanceScore)
        );
      }
      if (data.criteria?.length > 0) {
        setFields(data.criteria);
      }
      setJobCount(data.results?.length);
    } catch (error) {
      console.error(error);
    } finally {
      setLoading(false);
    }
  };

  return (
    <div className="grid w-full max-w-4xl gap-4 p-2 mx-auto antialiased font-normal leading-relaxed bg-white auto-rows-min scroll-smooth text-slate-800 sm:p-4">
      <div className="flex w-full gap-2 bg-white border-2 rounded flex-nowrap place-items-center border-aubergine">
        <div className="flex-1 px-3 py-2 text-2xl font-medium text-aubergine">
          NewExpress
        </div>
      </div>

      {fields &&
        fields.map((f, i) => (
          <div key={i} className="grid gap-4">
            <div key={i} className="flex flex-wrap gap-2">
              <div className="flex flex-wrap flex-1 px-4 py-2 rounded place-items-center bg-stone-100">
                <div className="flex-1">{f.name}</div>
                <div className="font-medium shrink-0">{f.criteria}</div>
              </div>
              <button
                onClick={() => handleEditClick(f, i)}
                className="btn-outline shrink-0"
              >
                <PencilSimple weight="bold" />
              </button>
              <button
                className="btn-outline k-delete shrink-0"
                onClick={(event) => handleDeleteClick(event, i)}
              >
                <TrashSimple weight="bold" className="text-orange-600" />
              </button>
            </div>
            <hr />
          </div>
        ))}

      <Transition.Root
        show={propertyMode === "edit" || propertyMode === "add"}
        as={Fragment}
      >
        <Dialog
          as="div"
          className="relative z-10 font-normal"
          onClose={() => {}}
        >
          <Transition.Child as={Fragment}>
            <div className="fixed inset-0 bg-gray-500 bg-opacity-75 transition-opacity" />
          </Transition.Child>

          <div className="fixed inset-0 z-10 overflow-y-auto font-sans">
            <div className="flex min-h-full items-end justify-center p-4 text-center sm:items-center sm:p-0">
              <Transition.Child as={Fragment}>
                <Dialog.Panel className="relative transform overflow-hidden rounded-lg bg-white px-4 pb-4 pt-5 text-left shadow-xl transition-all sm:my-8 sm:w-full sm:max-w-sm sm:p-6">
                  <div className="grid auto-rows-min gap-2 p-2">
                    <div className="text-xl font-medium">
                      {propertyMode === "edit"
                        ? "Edit a property"
                        : "Add a property"}
                    </div>
                    <hr className="my-2 w-full bg-slate-300" />
                    <div className="flex w-full gap-2">
                      <div className="flex-1 font-medium">
                        Name of the property
                      </div>
                    </div>
                    <input
                      type="text"
                      placeholder="E.g. experience"
                      value={formData.name}
                      onChange={(event) =>
                        setFormData({
                          ...formData,
                          name: event.target.value,
                        })
                      }
                      className="w-full"
                      disabled={true}
                    />
                    <hr className="my-2 w-full bg-slate-300" />
                    <div className="flex w-full gap-2">
                      <div className="flex-1 font-medium">
                        Value of the property
                      </div>
                    </div>
                    <input
                      type="text"
                      placeholder="E.g. embedded systems"
                      value={formData.criteria}
                      onChange={(event) =>
                        setFormData({
                          ...formData,
                          criteria: event.target.value,
                        })
                      }
                      className="w-full"
                    />
                    <hr className="my-2 w-full bg-slate-300" />
                    <div className="flex items-center">
                      <input
                        type="checkbox"
                        checked={formData.type === "or"}
                        onChange={(event) =>
                          setFormData({
                            ...formData,
                            type: event.target.checked ? "or" : "",
                          })
                        }
                        className="mr-2"
                      />
                      <div className="font-medium">
                        Mark as OR criteria(default is AND query)
                      </div>
                    </div>
                    <hr className="my-2 w-full bg-slate-300" />
                    <div className="flex w-full gap-1">
                      <button
                        onClick={() => handleSaveChangesClick(selectedProperty)}
                        className="btn-primary flex-1 !p-2 !text-sm font-medium"
                      >
                        Save changes
                      </button>
                      <button
                        onClick={handleCancelClick}
                        className="btn-outline flex-1 !p-2 !text-sm font-medium"
                      >
                        Cancel
                      </button>
                    </div>
                  </div>
                </Dialog.Panel>
              </Transition.Child>
            </div>
          </div>
        </Dialog>
      </Transition.Root>

      {/*<!-- spacer: mobile  -->*/}
      <div className="relative flex flex-wrap gap-4 pl-10">
        <div className="absolute top-0 left-0 p-2">
          <Plus weight="bold" className="text-stone-400" />
        </div>
        {/* dropdown TODO replace with variables*/}
        {suggestions.map((s, i) => (
          <button
            key={i}
            className="relative cursor-pointer btn-ghost"
            onClick={() => {
              setPropertyMode("add");
              setFormData({
                ...formData,
                name: s,
              });
            }}
          >
            {s}
          </button>
        ))}
        <button
          className="relative cursor-pointer btn-ghost"
          onClick={() => {
            setPropertyMode("add");
            setFormData({
              ...formData,
              name: "",
            });
          }}
        >
          Any property
        </button>
      </div>
      <button
        disabled={loading}
        onClick={handleSubmit}
        type="submit"
        className="btn-primary !p-3 text-xl !font-medium"
      >
        Tìm kiếm
      </button>
      <hr />
      {!loading && (
        <h3 className="text-xl font-medium">{jobCount} results found</h3>
      )}
      {/*<!-- block: ready to try? -->*/}
      <div className="">
        <div className="max-w-4xl">
          {loading && (
            <p className="text-center">
              Searching...
              <br /> Note: this can a few minutes...
            </p>
          )}
          {jobResults?.length > 0 && !loading && (
            <ul role="list" className="space-y-2">
              <PaginatedItems itemsPerPage={4} />,
            </ul>
          )}
          {jobResults?.length === 0 && loading === false && submitted && (
            <p className="text-center">No results</p>
          )}
        </div>
      </div>
    </div>
  );
}
