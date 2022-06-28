import json
from dataclasses import dataclass
from typing import Dict, List

import pandas as pd
import requests
from tqdm import tqdm


@dataclass
class ContractPackage:
    contract_package_hash: str
    owner_public_key: str
    contract_type_id: int
    contract_name: str
    contract_description: str
    timestamp: str

    @classmethod
    def from_json(cls, json_data):
        return cls(
            contract_package_hash=json_data["contract_package_hash"],
            owner_public_key=json_data["owner_public_key"],
            contract_type_id=json_data["contract_type_id"],
            contract_name=json_data["contract_name"],
            contract_description=json_data["contract_description"],
            timestamp=json_data["timestamp"]
        )


def get_page(base_url: str, page_number: int, limit: int = None) -> dict:
    """ Requests a page of data """
    if limit:
        url = base_url + f"?page={page_number}&limit={limit}"
    else:
        url = base_url + f"?page={page_number}"
    res = requests.get(url)
    return res.json()


def get_first_page(base_url: str) -> dict:
    """ Requests the first page of data """
    current_page = 1
    return get_page(base_url, current_page)


def read_page_count_from_data(data):
    return data["pageCount"]


def get_all_pages_data(base_url: str, page_count: int, limit: int) -> list:
    """ Requests all pages of data """
    all_pages_data = []
    for i in tqdm(range(1, page_count + 1)):
        all_pages_data.extend(get_page(base_url, i, limit)["data"])
    return all_pages_data


def retrieve_contracts(base_url: str, limit: int, contract_packages_file: str):
    """ Retrieves all contract packages from the API and writes them to a file """
    first_page = get_first_page(BASE_URL)
    page_count = read_page_count_from_data(first_page)
    all_pages_data = get_all_pages_data(BASE_URL, page_count, limit)

    with open(contract_packages_file, "w+") as f:
        json.dump(all_pages_data, f, indent=4)


def process_contracts(contract_packages_file: str):
    """ Loads the contract packages from a file and processes them """
    with open(contract_packages_file, "r") as f:
        data = json.load(f)

    # Map the data to a list of ContractPackage objects
    data = list(map(ContractPackage.from_json, data))

    print(f"Found {len(data)} contract packages")
    print(f"First contract package: {data[0]}")

    for package in data:
        if package.contract_name != None:
            print(
                f"{package.contract_package_hash} - {package.contract_name} - {package.contract_description}")


if __name__ == "__main__":
    PROTOCOL = "HTTPS://"
    DOMAIN = "event-store-api-clarity-testnet.make.services"
    ENDPOINT = "/contract-packages/"
    BASE_URL = PROTOCOL + DOMAIN + ENDPOINT
    LIMIT = 100
    CONTRACT_PACKAGES_FILE = "contract_packages.json"

    process_contracts(contract_packages_file=CONTRACT_PACKAGES_FILE)
